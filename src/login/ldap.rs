//! Login provider via LDAP.
//!
//! **Completely unimplemented!**

use db::Db;
use dict::{self, Locale};
use errors::*;
use login::{self, LoginError};
use user::{User, Role};

use ldap3::{LdapConn, Scope, SearchEntry};

pub struct Provider;

impl login::Provider for Provider {
    fn name(&self, locale: Locale) -> String {
        dict::new(locale).login.provider_name_ldap()
    }

    fn auth(&self, id: &str, secret: &str, db: &Db) -> Result<User> {
        // Open a connection to the LDAP server
        let ldap = LdapConn::new("ldaps://ldap.uni-osnabrueck.de")?;

        // Authenticate
        ldap.simple_bind(
            &format!("uid={},ou=people,dc=uni-osnabrueck,dc=de", id),
            secret
        )?
            .success()
            .chain_err(|| ErrorKind::LoginError(LoginError::SecretIncorrect))?;
            // TODO LoginError::CredentialsIncorrect

        // Find the user in the database..
        if let Some(user) = User::from_username(id, db)? {
            Ok(user)
        }

        // ... or create a new entry
        else {
            // Load the real name
            let (mut rs, _) = ldap.search(
                "ou=people,dc=uni-osnabrueck,dc=de",
                Scope::Subtree,
                &format!("uid={}", id),
                vec!["cn"]
            )?.success()?;
            let name = SearchEntry::construct(rs.remove(0))
                .attrs
                .remove("cn")
                .and_then(|mut v| {
                    if v.is_empty() {
                        None
                    } else {
                        Some(v.remove(0))
                    }
                });

            // Create the entry
            Ok(User::create(id.into(), name, Role::Student, db)?)
        }
    }
}
