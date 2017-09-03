use diesel;
use diesel::prelude::*;
use rocket::{Outcome, State};
use rocket::http::{Cookies, Status};
use rocket::request::{self, FromRequest, Request};
use std::ops::Deref;

use db::Db;
use db::schema::users;
use errors::*;
use login::Session;


pub mod routes;



#[derive(Debug, Clone, Eq, PartialEq, Identifiable, Queryable, Associations)]
pub struct User {
    /// Artificial unique ID. The `username` is already unique, but storing an
    /// integer is faster than a string.
    pub id: i64,

    /// Unique username for all users. This is most likely the "RZ Kennung".
    pub username: String,

    /// The real name of the user.
    pub name: Option<String>,

    /// The status of the user: student, tutor or administrator.
    pub role: Role,
}

impl User {
    pub fn from_username(username: &str, db: &Db) -> Result<Option<Self>> {
        // Find the user with the given username.
        users::table
            .filter(users::username.eq(username))
            .first(&*db.conn()?)
            .optional()?
            .make_ok()
    }

    pub fn create(username: String, name: Option<String>, db: &Db) -> Result<Self> {
        #[derive(Debug, Clone, Eq, PartialEq, Insertable)]
        #[table_name = "users"]
        struct NewUser {
            pub username: String,
            pub name: Option<String>,
        }

        let new_user = NewUser { username, name };

        diesel::insert(&new_user)
            .into(users::table)
            .get_result::<User>(&*db.conn()?)?
            .make_ok()
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

    pub fn destroy_session(self, cookies: Cookies, db: &Db) -> Result<()> {
        self.session.destroy(cookies, db)
    }
}

impl Deref for AuthUser {
    type Target = User;
    fn deref(&self) -> &Self::Target {
        &self.user
    }
}


impl<'a, 'r> FromRequest<'a, 'r> for AuthUser {
    type Error = Option<Error>;

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        // Obtain a DB pool.
        let db = req.guard::<State<Db>>().expect("cannot retrieve DB connection from request");

        match Session::from_cookies(req.cookies(), &db) {
            Err(e) => Outcome::Failure((Status::InternalServerError, Some(e))),
            Ok(Some(auth_user)) => Outcome::Success(auth_user),
            Ok(None) => Outcome::Failure((Status::Unauthorized, None)),
        }
    }
}


/// The role of the user.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Role {
    Admin,
    Tutor,
    Student,
}
