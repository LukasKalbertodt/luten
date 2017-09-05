use chrono::DateTime;
use chrono::offset::Utc;
use rocket::{Outcome, State};
use rocket::request::{self, FromRequest, Request};


use errors::*;
use db::Db;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    Preparation,
    Running,
    Frozen,
}


#[derive(Debug, Clone, Eq, PartialEq, Queryable)]
pub struct CurrentAppState {
    id: bool,
    pub state: AppState,
    pub reason: Option<String>,
    pub next_state_switch: Option<DateTime<Utc>>,
}

impl CurrentAppState {
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
        /// A request guard to ensure the current app state is "$variant".
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
