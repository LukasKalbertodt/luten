//! Defines the global `Error` and `Result` type and provides some helper
//! functionality for error handling.
//!
//! Please see the documentation of `error-chain` for more information. In
//! short: we define a big `Error` type here which can store everything that
//! can potentially go wrong. This means that we are using only one error type
//! in the whole program. Additionally, `Result` is redefined to include this
//! exact error type.

use diesel;
use pwhash;
use r2d2;
use rocket;
use std;

use dict::{self, Locale};
use login;


/// We will define our own `Result` type later. In order to still use the one
/// from `std`, we reexport it with another name. Same with `Error`.
pub use std::result::Result as StdResult;
pub use std::error::Error as StdError;

error_chain! {
    // All kinds of errors that can occur in this application
    foreign_links {
        // Errors that we don't expect to handle. But we also don't want to
        // panic. Instead, unhandled errors should result in a "500 internal
        // server error".
        DbPoolInit(r2d2::InitializationError);
        DbPoolTimeout(r2d2::GetTimeout);
        Db(diesel::result::Error);
        Hashing(pwhash::error::Error);
        Io(std::io::Error);

        // Our own errors (basically the recoverable ones)
        LoginError(login::LoginError);
    }

    errors {
        // Rocket HTTP status
        BadHttp(s: rocket::http::Status) {
            description("Rocket HTTP error")
        }
    }
}


/// This helper trait makes it possible to call `make_ok()` and `make_err()` on
/// all types.
///
/// The problem is, that wrapping stuff into `Ok()` or `Err()` is sometimes
/// really annoying if this stuff has multiple lines. Example:
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
/// nearly empty lines. Sure, one could use another strategy to break the line,
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
pub trait MakeResExt: Sized {
    fn make_ok<E>(self) -> StdResult<Self, E>;
    fn make_err<O>(self) -> StdResult<O, Self>;
}

impl<T> MakeResExt for T {
    fn make_ok<E>(self) -> StdResult<Self, E> {
        Ok(self)
    }
    fn make_err<O>(self) -> StdResult<O, Self> {
        Err(self)
    }
}

/// This method is used whenever the user sends a bad request which shouldn't
/// happen. It returns an error message to show to the user.
///
/// Evil users can just send arbitrary requests to `post` handlers. But for
/// normal users, they should be restricted by the HTML form. So if such a bad
/// request occurs, it is an indicator for a bug or an "attack".
pub fn bad_request(locale: Locale) -> String {
    // TODO: log potential bug
    dict::new(locale).bad_request_flash()
}
