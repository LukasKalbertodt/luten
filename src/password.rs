use diesel;
use diesel::prelude::*;
use pwhash::bcrypt;

use db::schema::passwords;
use db::Db;
use login::{self, LoginError};
use user::User;

#[derive(Debug, Clone, Eq, PartialEq, Identifiable, Queryable, Associations, Insertable)]
#[table_name = "passwords"]
#[primary_key(user_id)]
pub struct Password {
    user_id: i64,
    hash: String,
}

impl Password {
    pub fn create_for(user: &User, plain_pw: &str, db: &Db) -> Self {
        let password = bcrypt::hash(plain_pw).unwrap();
        let new = Self { user_id: user.id, hash: password };

        diesel::insert(&new)
            .into(passwords::table)
            .get_result::<Self>(&*db.conn())
            .unwrap()
    }

    pub fn load_of(user: &User, db: &Db) -> Option<Self> {
        passwords::table
            .find(user.id)
            .first::<Self>(&*db.conn())
            .optional()
            .unwrap()
    }

    pub fn verify(&self, password: &str) -> bool {
        bcrypt::verify(password, &self.hash)
    }
}

/// Authenticating users via passwords stored by this application.
pub struct InternalProvider;

impl login::Provider for InternalProvider {
    fn auth(&self, username: &str, secret: &str, db: &Db) -> Result<User, LoginError> {
        let user = User::from_username(username, db)
            .ok_or(LoginError::UserNotFound)?;

        let pw = Password::load_of(&user, db)
            .ok_or(LoginError::ProviderNotUsable)?;

        if pw.verify(secret) {
            Ok(user)
        } else {
            Err(LoginError::SecretIncorrect)
        }

    }
}
