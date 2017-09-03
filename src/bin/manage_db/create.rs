use clap::ArgMatches;
use rpassword;
use std::io::{self, Write};

use luten::db::Db;
use luten::errors::*;
use luten::user::{Role, User};

use db_util::find_user;
use util::{self, FromStdin, Global};


/// Create new entities in the database
pub fn create(util: &Global, matches: &ArgMatches, db: &Db) -> Result<()> {
    match matches.subcommand_name().unwrap() {
        "user" => {
            // In order to read the user role from terminal, we have to
            // implement this trait.
            impl FromStdin for Role {
                fn from_input(s: &str) -> StdResult<Self, String> {
                    match s {
                        "admin" => Ok(Role::Admin),
                        "tutor" => Ok(Role::Tutor),
                        "student" => Ok(Role::Student),
                        _ => Err("not a valid user role".into()),
                    }
                }
                fn desc() -> String {
                    "'UserRole' (one of 'admin', 'tutor', 'student')".into()
                }
            }


            println!("+-- Data for new user:");
            let result = User::create(
                util::read("username")?,
                util::read("name")?,
                util::read("role")?,
                db
            );
            println!("+-- Inserted:");
            util.debug_output(result);
        }
        "password" => {
            use luten::login::password::Password;

            println!("### First, choose the user you want to create a password for!");
            let user = find_user(db)?;
            println!("");

            println!("Creating password for:");
            util.debug_output(&user);
            println!("");

            print!("Type password (will be hidden): ");
            io::stdout().flush().unwrap();
            let pw = rpassword::read_password().unwrap();
            println!("");

            let result = Password::create_for(&user, &pw, db)?;

            println!("+-- Inserted:");
            util.debug_output(result);
        }
        _ => unreachable!(),
    }

    Ok(())
}
