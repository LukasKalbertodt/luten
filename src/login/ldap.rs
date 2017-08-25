//! Login provider via LDAP.
//!
//! **Completely unimplemented!**

use db::Db;
use errors::*;
use login::{self, LoginError};
use user::User;

pub struct Provider;

impl login::Provider for Provider {
    fn name(&self) -> String {
        "University-LDAP".into()
    }

    fn auth(&self, _id: &str, _secret: &str, _db: &Db) -> Result<User> {
        // TODO: impl

        bail!(LoginError::ProviderNotUsable)
    }
}
