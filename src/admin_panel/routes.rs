use std::collections::HashMap;

use diesel::prelude::*;
use diesel::result::{Error as DieselError, DatabaseErrorKind};
use option_filter::OptionFilterExt;
use rocket::State;
use rocket::config::Config;
use rocket::response::{Flash, Redirect};
use rocket::request::Form;

use db::Db;
use dict::{self, Locale};
use errors::*;
use state::{AppState, CurrentAppState};
use super::html;
use template::Page;
use timeslot::{self, DayOfWeek, NewTimeSlot, TimeSlot};
use user::{AuthAdmin, Role};


#[get("/admin_panel")]
pub fn index(
    _admin: AuthAdmin,
    locale: Locale,
    db: State<Db>,
    config: State<Config>,
) -> Result<Page> {
    use diesel::expression::dsl::*;
    use db::schema::users;

    // Calculate some stats.
    let counts = users::table
        .group_by(users::role)
        // We have to use raw sql here, because diesel is not powerful enough
        // to express this yet. See diesel-rs/diesel#772
        .select(sql("role, count(*)"))
        .load::<(Role, i64)>(&*db.conn()?)?
        .into_iter()
        .collect::<HashMap<_, _>>();

    let stats = html::Stats {
        num_admins: counts.get(&Role::Admin).cloned().unwrap_or(0) as u64,
        num_tutors: counts.get(&Role::Tutor).cloned().unwrap_or(0) as u64,
        num_students: counts.get(&Role::Student).cloned().unwrap_or(0) as u64,
    };

    Page::empty()
        .with_title("Admin Panel")
        .with_content(html::index(locale, &stats, &config))
        .with_active_nav_route("/admin_panel")
        .make_ok()
}


#[get("/admin_panel/state")]
pub fn state(
    _admin: AuthAdmin,
    locale: Locale,
    db: State<Db>,
) -> Result<Page> {
    let app_state = CurrentAppState::load(&db)?;

    Page::empty()
        .with_title(dict::new(locale).admin_panel.state_title())
        .with_active_nav_route("/admin_panel")
        .with_content(html::state(locale, &app_state))
        .make_ok()
}

#[derive(FromForm)]
pub struct StateChange {
    state: String,
    reason: Option<String>,
}

#[post("/admin_panel/state", data = "<form>")]
pub fn change_state(
    _admin: AuthAdmin,
    locale: Locale,
    form: Form<StateChange>,
    db: State<Db>,
) -> Result<Flash<Redirect>> {
    let dict = dict::new(locale).admin_panel;
    let form = form.into_inner();

    let state = match form.state.as_str() {
        "preparation" => AppState::Preparation,
        "running" => AppState::Running,
        "frozen" => AppState::Frozen,
        _ => {
            // Shouldn't happen unless the user sent invalid data.
            return Ok(Flash::error(
                Redirect::to("/admin_panel/state"),
                bad_request(locale),
            ));
        }
    };

    let reason = form.reason.filter(|r| !r.is_empty());

    // TODO: allow the user to specify the date
    CurrentAppState::set(state, reason, None, &db)?;

    Ok(Flash::success(
        Redirect::to("/admin_panel/state"),
        dict.flash_success_app_state_updated(),
    ))
}


#[get("/admin_panel/timeslots")]
pub fn timeslots(
    _admin: AuthAdmin,
    locale: Locale,
    db: State<Db>,
) -> Result<Page> {
    let mut timeslots = TimeSlot::load_all(&db)?;
    timeslots.sort();

    Page::empty()
        .with_title(dict::new(locale).admin_panel.timeslots_title())
        .with_active_nav_route("/admin_panel")
        .with_content(html::timeslots(&timeslots, locale))
        .make_ok()
}

#[derive(FromForm)]
pub struct NewTimeSlotForm {
    day: String,
    time: String,
}

#[post("/admin_panel/add_timeslot", data = "<form>")]
pub fn add_timeslot(
    _admin: AuthAdmin,
    form: Form<NewTimeSlotForm>,
    locale: Locale,
    db: State<Db>,
) -> Result<Flash<Redirect>> {
    let form = form.into_inner();

    let day = match DayOfWeek::from_variant_str(&form.day) {
        Some(day) => day,
        None => {
            return Ok(Flash::error(
                Redirect::to("/admin_panel/timeslots"),
                bad_request(locale),
            ));
        }
    };

    let times = match timeslot::parse_time_interval(&form.time) {
        Ok(times) => times,
        Err(e) => {
            return Ok(Flash::error(
                Redirect::to("/admin_panel/timeslots"),
                e,
            ));
        }
    };

    let count = times.len();
    let slots: Vec<_> = times.into_iter()
        .map(|time| NewTimeSlot::new(day, time))
        .collect();


    match TimeSlot::create_all(&slots, &db) {
        Ok(_) => {},
        Err(Error(ErrorKind::Db(
            DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _)
        ), _)) => {
            return Ok(Flash::error(
                Redirect::to("/admin_panel/timeslots"),
                "(Some of) the given timeslots already exist in the database! \
                    Please only add new ones!",
            ))
        }
        Err(e) => bail!(e),
    }

    Ok(Flash::success(
        Redirect::to("/admin_panel/timeslots"),
        format!("Successfully added {} new timeslots", count),
    ))
}

#[derive(FromForm)]
pub struct DeleteTimeSlotForm {
    id: i16,
}

#[post("/admin_panel/delete_timeslot", data = "<form>")]
pub fn delete_timeslot(
    _admin: AuthAdmin,
    form: Form<DeleteTimeSlotForm>,
    db: State<Db>,
) -> Result<Flash<Redirect>> {
    let form = form.into_inner();
    if TimeSlot::delete(form.id, &db)? {
        Ok(Flash::success(
            Redirect::to("/admin_panel/timeslots"),
            "Timeslot was successfully deleted",
        ))
    } else {
        Ok(Flash::success(
            Redirect::to("/admin_panel/timeslots"),
            "Timeslot wasn't delete since it doesn't exist.",
        ))
    }
}
