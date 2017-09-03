
use clap::ArgMatches;
use diesel::prelude::*;

use luten;
use luten::db::schema::{passwords, sessions, users};
use luten::db::Db;
use luten::errors::*;

use util::Global;

/// List entities from the database
pub fn list(util: &Global, matches: &ArgMatches, db: &Db) -> Result<()> {
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
        "passwords" => do_list!(passwords::table, luten::login::password::Password),
        _ => unreachable!(),
    }

    Ok(())
}
