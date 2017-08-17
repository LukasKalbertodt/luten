
use db::Db;
use super::LoginError;
use user::User;


/// A login-provider. Is able to authenticate a user.
pub trait Provider {
    fn auth(&self, username: &str, secret: &str, db: &Db) -> Result<User, LoginError>;
}


/// Authenticating users via passwords stored by this application.
pub struct Internal;

impl Provider for Internal {
    fn auth(&self, username: &str, secret: &str, db: &Db) -> Result<User, LoginError> {
        use diesel::prelude::*;
        use db::schema::passwords;
        use pwhash::bcrypt;

        let user = User::from_username(username, db).ok_or(LoginError::UserNotFound)?;

        #[derive(Debug, Clone, Eq, PartialEq, Identifiable, Queryable, Associations)]
        #[primary_key(user_id)]
        struct Password {
            user_id: i64,
            password: String,
        }

        let real_password = passwords::table
            .find(user.id)
            .first::<Password>(&*db.conn())
            .optional()
            .unwrap();

        let real_password = match real_password {
            None => return Err(LoginError::ProviderNotUsable),
            Some(rp) => rp.password,
        };

        if bcrypt::verify(secret, &real_password) {
            Ok(user)
        } else {
            Err(LoginError::SecretIncorrect)
        }
    }
}
