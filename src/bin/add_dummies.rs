//! Helper utility to add three dummy users to the database.
//!
//! Note that this doesn't add password logins for these users. Add those
//! manually via `manage_db create password`.

extern crate luten;
extern crate term_painter;


use std::cmp::min;

use term_painter::{Color, ToStyle};

use luten::db::Db;
use luten::errors::*;
use luten::user::{Role, User};
use luten::login::password::Password;


fn main() {
    // Establish DB connection
    let db = Db::open_connection();

    // Insert three users
    let do_it = || -> Result<()> {
        let student = User::create(
            "dummy_student".into(),
            Some("Willi Wacker".into()),
            Role::Student,
            &db,
        )?;
        let tutor = User::create(
            "dummy_tutor".into(),
            Some("Susi Sorglos".into()),
            Role::Tutor,
            &db,
        )?;
        let admin = User::create(
            "dummy_admin".into(),
            Some("Admin Peter".into()),
            Role::Admin,
            &db,
        )?;

        let dummy_password = "dummy";
        Password::create_for(&student, dummy_password, &db)?;
        Password::create_for(&tutor, dummy_password, &db)?;
        Password::create_for(&admin, dummy_password, &db)?;


        println!("Inserted:");
        println!("'{:?}' with password '{}'", student, dummy_password);
        println!("'{:?}' with password '{}'", tutor, dummy_password);
        println!("'{:?}' with password '{}'", admin, dummy_password);

        Ok(())
    };

    // Pretty print error chain
    if let Err(error_chain) = do_it() {
        println!("Something went wrong ☹ ! Here is the backtrace:");
        for (i, e) in error_chain.iter().enumerate() {
            println!(
                "{: >2$} {}",
                Color::Yellow.paint(if i == 0 { "→" } else { "⤷" }),
                Color::Red.paint(e),
                2 * min(i, 7) + 1,
            );
        }
    }
}
