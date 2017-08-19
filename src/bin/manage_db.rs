#[macro_use] extern crate clap;
extern crate diesel;
extern crate luten;
extern crate rpassword;
extern crate term_painter;


use std::cmp::min;
use std::fmt::Debug;
use std::io::{self, Write};

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use diesel::prelude::*;
use term_painter::{Color, ToStyle};

use luten::db::schema::{passwords, sessions, users};
use luten::db::Db;
use luten::errors::*;
use luten::user::User;


fn main() {
    // Define CLI and parse arguments
    let matches = App::new("manage_db")
        .about("Manage entities in the luten database")
        .setting(AppSettings::SubcommandRequired)
        .arg(
            Arg::with_name("compact")
                .short("c")
                .help("Enables compact output mode")
        )
        .subcommands(vec![
            SubCommand::with_name("list")
                .about("List database entries")
                .setting(AppSettings::SubcommandRequired)
                .args(&[
                    Arg::with_name("limit")
                        .long("limit")
                        .takes_value(true),
                    Arg::with_name("offset")
                        .long("offset")
                        .takes_value(true),
                ])
                .subcommands(vec![
                    SubCommand::with_name("users"),
                    SubCommand::with_name("sessions"),
                    SubCommand::with_name("passwords"),
                ]),
            SubCommand::with_name("create")
                .about("Create new entities in the database")
                .setting(AppSettings::SubcommandRequired)
                .subcommands(vec![
                    SubCommand::with_name("user"),
                    SubCommand::with_name("password"),
                ])
        ])
        .get_matches();

    // Establish DB connection
    let db = Db::open_connection();

    // Create Util object
    let util = Util {
        compact_output: matches.is_present("compact"),
    };

    // We can unwrap(), because we set "SubcommandRequired" above.
    let (sub_name, sub_matches) = matches.subcommand();
    let sub_matches = sub_matches.unwrap();
    let res = match sub_name {
        "list" => list(&util, &sub_matches, &db),
        "create" => create(&util, &sub_matches, &db),
        _ => unreachable!(),
    };

    // Pretty print error chain
    if let Err(error_chain) = res {
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

macro_rules! value_t_opt {
    ($matches:ident, $name:expr, $ty:ty) => {
        if $matches.is_present($name) {
            Some(value_t!($matches, $name, $ty).unwrap_or_else(|e| e.exit()))
        }  else {
            None
        }
    }
}

// ==========================================================================
// Subcommands
// ==========================================================================

/// List entities from the database
fn list(util: &Util, matches: &ArgMatches, db: &Db) -> Result<()> {
    /// Helper macro to avoid duplicate code
    macro_rules! do_list {
        ($table:path, $ty:path) => {{

            let limit = value_t_opt!(matches, "limit", i64).unwrap_or(i64::max_value());
            let offset = value_t_opt!(matches, "offset", i64).unwrap_or(0);

            let result = $table
                .limit(limit)
                .offset(offset)
                .load::<$ty>(&*db.conn()?)?;

            println!("### Results ({}):", result.len());
            for row in result {
                util.debug_output(row);
            }
        }}
    }

    match matches.subcommand_name().unwrap() {
        "users" => do_list!(users::table, luten::user::User),
        "sessions" => do_list!(sessions::table, luten::login::Session),
        "passwords" => do_list!(passwords::table, luten::password::Password),
        _ => unreachable!(),
    }

    Ok(())
}

/// Create new entities in the database
fn create(util: &Util, matches: &ArgMatches, db: &Db) -> Result<()> {
    match matches.subcommand_name().unwrap() {
        "user" => {
            println!("+-- Data for new user:");
            let result = User::create(
                read("username"),
                read("name"),
                db
            );
            println!("+-- Inserted:");
            util.debug_output(result);
        }
        "password" => {
            use luten::password::Password;

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

fn find_user(db: &Db) -> Result<User> {
    loop {
        println!("Specify a user by providing:");
        println!(" - the user id in '#id' syntax (e.g. '#13'), or");
        println!(" - the username in '@username' syntax (e.g. '@xmuster')");

        let line = read_trimmed_line();
        match line.chars().next() {
            Some('#') => {
                let user_id = match line[1..].parse::<i64>() {
                    Ok(v) => v,
                    Err(e) => {
                        println!("Invalid user id: {}", e);
                        continue;
                    }
                };

                let result = users::table
                    .find(user_id)
                    .first(&*db.conn()?)
                    .optional()?;

                match result {
                    Some(u) => return Ok(u),
                    None => {
                        println!("No user with id {} found!", user_id);
                    }
                }

            }
            Some('@') => {}
            _ => {
                println!("Invalid input! Use the syntax as described above!");
            }
        }
    }
}


// ==========================================================================
// Helper types and functions
// ==========================================================================

fn read_trimmed_line() -> String {
    let mut out = String::new();
    io::stdin().read_line(&mut out).unwrap();
    out.trim().to_owned()
}

/// A helper struct which holds the global config. Actions influenced by the
/// global config should be done through this struct.
struct Util {
    compact_output: bool,
}

impl Util {
    fn debug_output<T: Debug>(&self, v: T) {
        if self.compact_output {
            println!("{:?}", v);
        } else {
            println!("{:#?}", v);
        }
    }
}

/// Reads a type from stdin with a proper message.
fn read<T: FromStdin>(field_name: &str) -> T {
    loop {
        print!("| {} ({}): ", field_name, <T as FromStdin>::desc());
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        match T::from_input(line.trim()) {
            Ok(v) => return v,
            Err(e) => println!("Input error: {}", e),
        }
    }
}

/// Types that can be read from stdin.
trait FromStdin: Sized {
    fn from_input(s: &str) -> StdResult<Self, String>;
    fn desc() -> String;
}

macro_rules! impl_from_stdin {
    ($ty:ident) => {
        impl FromStdin for $ty {
            fn from_input(s: &str) -> StdResult<Self, String> {
                s.parse::<$ty>().map_err(|e| e.to_string())
            }
            fn desc() -> String {
                format!("`{}`", stringify!($ty))
            }

        }

        impl FromStdin for Option<$ty> {
            fn from_input(s: &str) -> StdResult<Self, String> {
                if s.is_empty() {
                    Ok(None)
                } else {
                    s.parse::<$ty>()
                        .map(|v| Some(v))
                        .map_err(|e| e.to_string())
                }
            }

            fn desc() -> String {
                format!("`Option<{}>`, empty input for `None`", stringify!($ty))
            }
        }
    }
}

impl_from_stdin!(u8);
impl_from_stdin!(u16);
impl_from_stdin!(u32);
impl_from_stdin!(u64);
impl_from_stdin!(i8);
impl_from_stdin!(i16);
impl_from_stdin!(i32);
impl_from_stdin!(i64);
impl_from_stdin!(String);
