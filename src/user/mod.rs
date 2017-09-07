//! User handling. **Has routes**.
//!
//! This module mainly consists of functionality to work with users. It has a
//! few routes, thought:
//!
//! - GET `/settings`: user specific settings

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


/// A representation of a user in the database.
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
    /// Tries to find a user with the given username in the database and
    /// returns it.
    pub fn load_by_username(username: &str, db: &Db) -> Result<Option<Self>> {
        // Find the user with the given username.
        users::table
            .filter(users::username.eq(username))
            .first(&*db.conn()?)
            .optional()?
            .make_ok()
    }

    /// Creates a new user from the given data and stores it in the database.
    pub fn create(
        username: String,
        name: Option<String>,
        role: Role,
        db: &Db,
    ) -> Result<Self> {
        #[derive(Debug, Clone, Eq, PartialEq, Insertable)]
        #[table_name = "users"]
        struct NewUser {
            pub username: String,
            pub name: Option<String>,
            pub role: Role,
        }

        let new_user = NewUser { username, name, role };

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

    pub fn role(&self) -> Role {
        self.role
    }

    pub fn is_admin(&self) -> bool {
        self.role == Role::Admin
    }

    pub fn is_tutor(&self) -> bool {
        self.role == Role::Tutor
    }

    pub fn is_student(&self) -> bool {
        self.role == Role::Student
    }

    pub fn into_admin(self) -> StdResult<Admin, Self> {
        if self.is_admin() {
            Ok(Admin(self))
        } else {
            Err(self)
        }
    }

    pub fn into_tutor(self) -> StdResult<Tutor, Self> {
        if self.is_tutor() {
            Ok(Tutor(self))
        } else {
            Err(self)
        }
    }

    pub fn into_student(self) -> StdResult<Student, Self> {
        if self.is_student() {
            Ok(Student(self))
        } else {
            Err(self)
        }
    }
}

/// The role of the user.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Role {
    Admin,
    Tutor,
    Student,
}

macro_rules! create_user_role_type {
    ($role:ident) => {
        #[derive(Debug, Clone)]
        pub struct $role(User);

        impl $role {
            pub fn id(&self) -> i64 {
                self.0.id
            }

            pub fn username(&self) -> &str {
                &self.0.username
            }

            pub fn name(&self) -> Option<&str> {
                self.0.name.as_ref().map(AsRef::as_ref)
            }

            pub fn into_inner(self) -> User {
                self.0
            }
        }
    }
}

create_user_role_type!(Student);
create_user_role_type!(Tutor);
create_user_role_type!(Admin);

/// An authorized user with an active session. This type doesn't restrict
/// access to any properties, as the user is logged in.
///
/// This type implements `FromRequest` and can thus be easily obtain in any
/// handler. The `from_request()` call fails if there is no user logged in.
/// This way, handlers can easily ensure that they can only be visited with a
/// valid login session.
#[derive(Debug, Clone, Eq, PartialEq)]
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
            Ok(None) => Outcome::Failure((Status::Forbidden, None)),
        }
    }
}


/// An authorized user which has the user role `Admin`.
///
/// This type implements `FromRequest` and can therefore be used as request
/// guard.
pub struct AuthAdmin(pub AuthUser);

impl Deref for AuthAdmin {
    type Target = AuthUser;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthAdmin {
    type Error = Option<Error>;

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let user = AuthUser::from_request(req)?;
        if user.role() == Role::Admin {
            Outcome::Success(AuthAdmin(user))
        } else {
            Outcome::Failure((Status::Unauthorized, None))
        }
    }
}
