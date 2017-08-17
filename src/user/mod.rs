use diesel::prelude::*;
use rocket::{Outcome, State};
use rocket::http::{Cookies, Status};
use rocket::request::{self, FromRequest, Request};
use std::ops::Deref;

use db::Db;
use db::schema::users;
use login::Session;




#[derive(Debug, Clone, Eq, PartialEq, Identifiable, Queryable, Associations)]
pub struct User {
    /// Artificial unique ID. The `username` is already unique, but storing an
    /// integer is faster than a string.
    pub id: i64,

    /// Unique username for all users. This is most likely the "RZ Kennung".
    pub username: String,

    /// The real name of the user.
    pub name: Option<String>,
}

impl User {
    pub fn from_username(username: &str, db: &Db) -> Option<Self> {
        // Find the user with the given username.
        users::table
            .filter(users::username.eq(username))
            .limit(1)
            .first(&*db.conn())
            .optional()
            .unwrap()
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(AsRef::as_ref)
    }
}

/// An authorized user with an active session. This type doesn't restrict
/// access to any properties, as the user is logged in.
#[derive(Clone, Eq, PartialEq)]
pub struct AuthUser {
    user: User,
    session: Session,
}

impl AuthUser {
    pub fn new(user: User, session: Session) -> Self {
        Self { user, session }
    }

    pub fn destroy_session(self, cookies: Cookies, db: &Db) {
        self.session.destroy(cookies, db);
    }

    // /// Ends a login session, removing the entry from the database and removing
    // /// the cookie.
    // ///
    // /// This function assumes the user was authenticated via session cookie.
    // pub fn end_session(&self, cookies: &Cookies, db: &Db) {
    //     // Since we assume the user was authenticated via session id, we know
    //     // the cookie jar contains such a cookie and the cookie is a valid
    //     // hex string.
    //     let session_id = hex::decode(
    //         cookies.find(SESSION_COOKIE_NAME).unwrap().value()
    //     ).unwrap();

    //     // Remove from database.
    //     diesel::delete(sessions::table.find(session_id))
    //         .execute(&*db.conn())
    //         .expect("failed to delete session entry from database");

    //     // Remove from cookie jar.
    //     cookies.remove(SESSION_COOKIE_NAME);
    // }
}

impl Deref for AuthUser {
    type Target = User;
    fn deref(&self) -> &Self::Target {
        &self.user
    }
}


impl<'a, 'r> FromRequest<'a, 'r> for AuthUser {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        // Obtain a DB pool.
        let db = req.guard::<State<Db>>().expect("cannot retrieve DB connection from request");

        Session::verify(req.cookies(), &db)
            .map(|auth_user| Outcome::Success(auth_user))
            .unwrap_or(
                Outcome::Failure(
                    (Status::Unauthorized, ())
                )
            )
    }
}
