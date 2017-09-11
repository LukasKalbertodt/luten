//! Login provider via LDAP.
//!
//! **Completely unimplemented!**

use ldap3::{LdapConn, Scope, SearchEntry};

use db::Db;
use dict::{self, Locale};
use errors::*;
use login::{self, LoginError};
use user::{User, Role};

const LDAP_URL: &'static str = "ldaps://ldap.uni-osnabrueck.de";
const LDAP_BASE: &'static str = "ou=people,dc=uni-osnabrueck,dc=de";
const LDAP_CN: &'static str = "cn";
const LDAP_UID: &'static str = "uid";

pub struct Provider;

impl login::Provider for Provider {
    fn name(&self, locale: Locale) -> String {
        dict::new(locale).login.provider_name_ldap()
    }

    fn auth(&self, id: &str, secret: &str, db: &Db) -> Result<User> {
        // Open a connection to the LDAP server
        let ldap = LdapConn::new(LDAP_URL)?;

        // Authenticate
        ldap.simple_bind(
            &format!("{}={},{}", LDAP_UID, id, LDAP_BASE),
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
                LDAP_BASE,
                Scope::Subtree,
                &format!("{}={}", LDAP_UID, id),
                vec![LDAP_CN]
            )?.success()?;
            let name = SearchEntry::construct(rs.remove(0))
                .attrs
                .remove(LDAP_CN)
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
