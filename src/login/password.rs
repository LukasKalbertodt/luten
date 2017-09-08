//! Module about webapp internal passwords.
//!
//! This web app can store passwords for users as well. This is usually only
//! used for users in development or for dummy-users. Real users should
//! probably be authenticated via LDAP or something like that.

use diesel;
use diesel::prelude::*;
use pwhash::bcrypt;

use db::schema::passwords;
use db::Db;
use dict::{self, Locale};
use errors::*;
use login::{self, LoginError};
use user::User;


/// A bcrypt-hashed password.
///
/// Passwords live in their own table and *not* in the `users` table.
#[derive(Debug, Clone, Eq, PartialEq, Identifiable, Queryable, Associations, Insertable)]
#[table_name = "passwords"]
#[primary_key(user_id)]
pub struct Password {
    user_id: i64,
    hash: String,
}

impl Password {
    /// Creates a password from the given plain password string for the given
    /// user and inserts it into the database.
    ///
    /// Errors if the user already has a password.
    pub fn create_for(user: &User, plain_pw: &str, db: &Db) -> Result<Self> {
        let password = Self::hash_of(plain_pw)?;
        let new = Self {
            user_id: user.id,
            hash: password
        };

        diesel::insert(&new)
            .into(passwords::table)
            .get_result::<Self>(&*db.conn()?)
            .chain_err(|| "Error inserting a new password")?
            .make_ok()
    }

    /// Returns the raw hash for the given plain text password. This hash will
    /// be stored in the database when `create_for()` is called.
    pub fn hash_of(plain_pw: &str) -> Result<String> {
        bcrypt::hash(plain_pw).map_err(|e| e.into())
    }

    /// Loads the password of the given user from the database. Returns `None`
    /// if there is no password associated with this user.
    pub fn load(user: &User, db: &Db) -> Result<Option<Self>> {
        passwords::table
            .find(user.id)
            .first::<Self>(&*db.conn()?)
            .optional()?
            .make_ok()
    }

    /// Check if the given plain password matches this hashed password.
    pub fn verify(&self, password: &str) -> bool {
        bcrypt::verify(password, &self.hash)
    }
}

/// Authenticating users via passwords stored by this application.
pub struct Provider;

impl login::Provider for Provider {
    fn name(&self, locale: Locale) -> String {
        dict::new(locale).login.provider_name_password()
    }

    fn auth(&self, id: &str, secret: &str, db: &Db) -> Result<User> {
        // TODO: here we have two queries, although we could get the same
        // information using a join in one query. We might want to change this.
        let user = User::from_username(id, db)?
            .ok_or(LoginError::UserNotFound)?;

        let pw = Password::load(&user, db)?
            .ok_or(LoginError::ProviderNotUsable)?;

        if pw.verify(secret) {
            Ok(user)
        } else {
            bail!(LoginError::SecretIncorrect)
        }
    }
}
