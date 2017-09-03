//! Reads a password from stdin and prints its hash (the value that would be
//! stored in the database).
extern crate luten;
extern crate rpassword;
extern crate term_painter;


use std::cmp::min;
use std::io::{self, Write};

use term_painter::{Color, ToStyle};

use luten::login::password::Password;



fn main() {
    println!("Will calculate the hash of a given password!");
    print!("Type password (will be hidden): ");
    io::stdout().flush().unwrap();
    let pw = rpassword::read_password().unwrap();
    println!("");

    match Password::hash_of(&pw) {
        Ok(hash) => println!("Hash: {}", hash),
        Err(error_chain) => {
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
}
