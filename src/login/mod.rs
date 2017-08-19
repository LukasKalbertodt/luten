use chrono::DateTime;
use chrono::offset::Utc;
use diesel::prelude::*;
use diesel;
use hex;
use option_filter::OptionFilterExt;
use rand::{self, Rng};
use rocket::http::{Cookie, Cookies};
use std::fmt;

use config;
use db::Db;
use db::schema::{sessions, users};
use errors::*;
use user::{AuthUser, User};

pub mod routes;
mod html;


/// A login-provider. Is able to authenticate a user.
pub trait Provider {
    fn auth(&self, username: &str, secret: &str, db: &Db) -> Result<User>;
}



pub fn login(
    username: &str,
    secret: &str,
    provider: &Provider,
    cookies: Cookies,
    db: &Db,
) -> Result<AuthUser> {
    // Try to authenticate with the given provider. If it fails, we return an
    // error.
    let user = provider.auth(username, secret, db)?;

    // Create a session in the database and set it as cookie.
    let session = Session::create_for(&user, cookies, db)?;

    Ok(AuthUser::new(user, session))
}


#[derive(Debug)]
pub enum LoginError {
    /// There is not user with the given username.
    UserNotFound,

    /// A user was found, but the given password/secret is not correct.
    SecretIncorrect,

    /// A user was found, but cannot be authenticated with this provider.
    ProviderNotUsable,
}

impl fmt::Display for LoginError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(f)
    }
}

impl StdError for LoginError {
    fn description(&self) -> &'static str {
        use self::LoginError::*;

        match *self {
            UserNotFound => "the given user was not found",
            SecretIncorrect => "the given secret is not correct",
            ProviderNotUsable => "the given user cannot be authenticated with this provider",
        }
    }
}




#[derive(Debug, Clone, Eq, PartialEq, Identifiable, Queryable, Associations)]
// #[belongs_to(User)]
pub struct Session {
    pub id: Vec<u8>,
    pub user_id: i64,
    pub birth: DateTime<Utc>,
}

impl Session {
    pub fn create_for(user: &User, mut cookies: Cookies, db: &Db) -> Result<Self> {
        // Generate a random session id.
        let mut id = [0u8; config::SESSION_ID_LEN];
        let mut rng = rand::os::OsRng::new()
            .chain_err(|| "Unable to use system RNG")?;
        rng.fill_bytes(&mut id);

        // Insert session id linked with the user id into the database.
        #[derive(Debug, Clone, Eq, PartialEq, Insertable)]
        #[table_name = "sessions"]
        pub struct NewSession {
            pub id: Vec<u8>,
            pub user_id: i64,
        }

        let new_session = NewSession {
            id: id.to_vec(),
            user_id: user.id(),
        };
        let inserted_session = diesel::insert(&new_session)
            .into(sessions::table)
            .get_result::<Session>(&*db.conn()?)?;

        // Encode session id as hex and set it as cookie.
        let encoded = hex::encode(&id);
        cookies.add(Cookie::new(config::SESSION_COOKIE_NAME, encoded));

        Ok(inserted_session)
    }

    pub fn verify(cookies: Cookies, db: &Db) -> Result<Option<AuthUser>> {
        // TODO: once associations work again, use a join here instead of two
        // queries.

        let session_id = cookies.get(config::SESSION_COOKIE_NAME)
            .and_then(|cookie| hex::decode(cookie.value()).ok())
            .filter(|session_id| session_id.len() == config::SESSION_ID_LEN);

        let session_id = try_opt_ok!(session_id);


        // Try to find a session with the given id
        let session = sessions::table
            .find(session_id)
            .first::<Session>(&*db.conn()?)
            .optional()?;
        let session = try_opt_ok!(session);

        // Try to find the user referenced by that session. If found, combine
        // that user with the session to make an `AuthUser`.
        users::table
            .find(session.user_id)
            .first::<User>(&*db.conn()?)
            .optional()?
            .map(|user| AuthUser::new(user, session))
            .make_ok()
    }

    /// Ends a login session, removing the entry from the database and removing
    /// the cookie.
    ///
    /// This function assumes the user was authenticated via session cookie.
    pub fn destroy(self, mut cookies: Cookies, db: &Db) -> Result<()> {
        // Since we assume the user was authenticated via session id, we know
        // the cookie jar contains such a cookie and the cookie is a valid
        // hex string.
        let session_id = hex::decode(
            cookies.get(config::SESSION_COOKIE_NAME).unwrap().value()
        ).unwrap();

        // Remove from database.
        diesel::delete(sessions::table.find(session_id))
            .execute(&*db.conn()?)?;

        // Remove from cookie jar.
        cookies.remove(Cookie::named(config::SESSION_COOKIE_NAME));

        Ok(())
    }
}
