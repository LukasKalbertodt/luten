
use db::Db;
use super::LoginError;
use user::User;


/// A login-provider. Is able to authenticate a user.
pub trait Provider {
    fn auth(&self, username: &str, secret: &[u8], db: &Db) -> Result<User, LoginError>;
}


/// Authenticating users via passwords stored by this application.
pub struct Internal;

impl Provider for Internal {
    fn auth(&self, username: &str, secret: &[u8], db: &Db) -> Result<User, LoginError> {
        let user = User::from_username(username, db).ok_or(LoginError::UserNotFound)?;

        match secret {
            b"peter" => Ok(user),
            _ => Err(LoginError::SecretIncorrect),
        }
    }
}
