//! The state of the application.
//!
//! The website can be in different states. The current state influences which
//! routes are available. The two main states are:
//!
//! - `Preparation`: this is where tutors and students specify their
//!   preferences regarding time slots, ... This state is active only briefly
//!   at the beginning of the semester.
//! - `Running`: when all time slots and partners/tutors are assigned. This is
//!   active during the semester.

use chrono::DateTime;
use chrono::offset::Utc;
use rocket::{Outcome, State};
use rocket::request::{self, FromRequest, Request};


use errors::*;
use db::Db;

/// All possible states of the app
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    Preparation,
    Running,

    /// This is a helper state for administrators. It disable all meaningful
    /// routes and can be used to ... calculate a schedule or stuff like that.
    Frozen,
}


/// Represents the current application state, as stored in the database.
#[derive(Debug, Clone, Eq, PartialEq, Queryable)]
pub struct CurrentAppState {
    id: bool,
    pub state: AppState,
    pub reason: Option<String>,
    pub next_state_switch: Option<DateTime<Utc>>,
}

impl CurrentAppState {
    /// Loads the current app state from the database.
    pub fn load(db: &Db) -> Result<Self> {
        use db::schema::current_app_state;
        use diesel::prelude::*;

        current_app_state::table
            .first::<Self>(&*db.conn()?)?
            .make_ok()
    }

    pub fn reason(&self) -> Option<&str> {
        self.reason.as_ref().map(AsRef::as_ref)
    }
}


macro_rules! state_req_guard {
    ($name:ident, $variant:ident) => {
        /// A request guard to ensure the app is in a specific state.
        pub struct $name(pub CurrentAppState);

        impl<'a, 'r> FromRequest<'a, 'r> for $name {
            type Error = ();

            fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
                let db = req.guard::<State<Db>>().expect("cannot retrieve DB connection from request");
                let app_state = CurrentAppState::load(&db);
                if let Ok(app_state) = app_state {
                    if app_state.state == AppState::$variant {
                        return Outcome::Success($name(app_state));
                    }
                }
                Outcome::Forward(())
            }
        }
    }
}

state_req_guard!(PreparationState, Preparation);
state_req_guard!(RunningState, Running);
state_req_guard!(FrozenState, Frozen);
