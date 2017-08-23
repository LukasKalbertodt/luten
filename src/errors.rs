#![allow(unused_doc_comment)]
/// ^ Otherwise, `error_chain!()` generates a warning :/

use diesel;
use pwhash;
use r2d2;
use std;

use login;


/// We will define our own `Result` type later. In order to still use the one
/// from `std`, we reexport it with another name. Same with `Error`.
pub use std::result::Result as StdResult;
pub use std::error::Error as StdError;

error_chain! {
    // All kinds of errors that can occur in this application
    foreign_links {
        DbPoolInit(r2d2::InitializationError);
        DbPoolTimeout(r2d2::GetTimeout);
        Db(diesel::result::Error);
        Hashing(pwhash::error::Error);
        Io(std::io::Error);

        // Our own errors (basically the recoverable ones)
        LoginError(login::LoginError);
    }
}


/// This helper traits makes it possible to call `make_ok()` on all types.
///
/// The problem is, that wrapping stuff into `Ok()` is sometimes really
/// annoying if this stuff has multiple lines. Example:
///
/// ```ignore
/// Ok(
///     users::table
///         .filter(users::username.eq(username))
///         .first(&*db.conn()?)
///         .optional()?
/// )
/// ```
///
/// This way, the whole thing is indented one additional level and we have two
/// nearly empty line. Sure, one could use another strategy to break the line,
/// but it's still ugly.
///
/// So instead, we can write this:
///
/// ```ignore
/// users::table
///     .filter(users::username.eq(username))
///     .first(&*db.conn()?)
///     .optional()?
///     .make_ok()
/// ```
///
/// Maybe it's questionable if this is a good solution, but I like it a lot
/// better. This shouldn't be use for situations in which `Ok()` is fine!
pub trait MakeOkExt: Sized {
    fn make_ok<E>(self) -> StdResult<Self, E>;
}

impl<T> MakeOkExt for T {
    fn make_ok<E>(self) -> StdResult<Self, E> {
        Ok(self)
    }
}
