use clap::ArgMatches;
use diesel::prelude::*;

use luten::db::Db;
use luten::db::schema::{prep_student_preferences, users};
use luten::errors::*;
use luten::prep::StudentPreferences;
use luten::user::User;

use util::Global;


/// Fix broken stuff in the database.
pub fn fix(_glob: &Global, matches: &ArgMatches, db: &Db) -> Result<()> {
    match matches.subcommand_name().unwrap() {
        "missing_prep_preferences" => {
            use diesel::expression::dsl::sql;

            // All students
            let res = users::table
                .filter(sql("role='student'"))
                .left_join(prep_student_preferences::table)
                .filter(sql("user_id is null"))
                .load::<(User, Option<StudentPreferences>)>(&*db.conn()?)?;

            for (user, _) in res {
                let student = user.into_student().unwrap();
                StudentPreferences::create_default(&student, db)?;
                println!("Created preferences for #{} (@{})", student.id(), student.username());
            }
        }
        _ => unreachable!(),
    }

    Ok(())
}
