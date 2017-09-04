use chrono::DateTime;
use chrono::offset::Utc;

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
}
