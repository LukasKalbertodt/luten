//! Command line utility to interact with the database used by the web app.
//!
//! We don't have anything as fancy as `$ rails c`, but we can write our own
//! little utility for managing entities in our database.
//!
//! For more information on how to use this, execute:
//!
//! ```none
//! $ cargo run --bin manage_db -- -h
//! ```
#[macro_use] extern crate clap;
extern crate diesel;
extern crate luten;
extern crate rpassword;
extern crate term_painter;


use std::cmp::min;

use clap::{App, AppSettings, Arg, SubCommand};
use term_painter::{Color, ToStyle};

use luten::db::Db;



#[macro_use]
mod util;

mod create;
mod db_util;
mod fix;
mod list;


fn main() {
    // Define CLI and parse arguments.
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
                ]),
            SubCommand::with_name("fix")
                .about("Fixes several inconsistencies in the database")
                .setting(AppSettings::SubcommandRequired)
                .subcommands(vec![
                    SubCommand::with_name("missing_prep_preferences")
                        .about(
                            "Each user should have prep-preferences associated with it. If that's \
                             not the case, this command will add default preferences."
                        ),
                ]),
        ])
        .get_matches();

    // Establish DB connection
    let db = Db::open_connection();

    // Create Global object
    let util = util::Global {
        compact_output: matches.is_present("compact"),
    };

    // We can unwrap(), because we set "SubcommandRequired" above.
    let (sub_name, sub_matches) = matches.subcommand();
    let sub_matches = sub_matches.unwrap();
    let res = match sub_name {
        "list" => list::list(&util, &sub_matches, &db),
        "create" => create::create(&util, &sub_matches, &db),
        "fix" => fix::fix(&util, &sub_matches, &db),
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
