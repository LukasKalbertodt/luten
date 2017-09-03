use diesel::prelude::*;

use luten::db::Db;
use luten::db::schema::users;
use luten::errors::*;
use luten::user::User;

use util;


/// Lets the user interactively specify a user.
pub fn find_user(db: &Db) -> Result<User> {
    loop {
        println!("Specify a user by providing:");
        println!(" - the user id in '#id' syntax (e.g. '#13'), or");
        println!(" - the username in '@username' syntax (e.g. '@xmuster')");

        let line = util::read_trimmed_line()?;
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
            Some('@') => {
                // TODO
            }
            _ => {
                println!("Invalid input! Use the syntax as described above!");
            }
        }
    }
}
