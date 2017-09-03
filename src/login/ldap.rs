//! Login provider via LDAP.
//!
//! **Completely unimplemented!**

use db::Db;
use dict::{self, Locale};
use errors::*;
use login::{self, LoginError};
use user::User;

pub struct Provider;

impl login::Provider for Provider {
    fn name(&self, locale: Locale) -> String {
        dict::new(locale).login.provider_name_ldap()
    }

    fn auth(&self, _id: &str, _secret: &str, _db: &Db) -> Result<User> {
        // TODO: impl

        bail!(LoginError::ProviderNotUsable)
    }
}
