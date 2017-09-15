use rocket::State;
use rocket::response::{Flash, Redirect};
use rocket::request::{Form, FormItems, FromForm};
use option_filter::OptionFilterExt;

use super::{html, StudentPreferences, TimeSlotRating};
use config;
use db::Db;
use dict::{self, Locale};
use errors::*;
use state::PreparationState;
use template::{NavItem, Page};
use user::{AuthUser, Role, User};
use timeslot::Rating;


fn nav_items(locale: Locale) -> Vec<NavItem> {
    // TODO: pass `Dict` once possible
    let dict = dict::new(locale).prep;

    vec![
        NavItem::new(dict.nav_overview_title(), "/prep"),
        NavItem::new(dict.nav_timeslots_title(), "/prep/timeslots"),
    ]
}

#[get("/prep")]
pub fn overview(
    auth_user: AuthUser,
    locale: Locale,
    db: State<Db>,
    _state: PreparationState,
) -> Result<Page> {
    let dict = dict::new(locale).prep;

    match auth_user.role() {
        // ===== Student ======================================================
        Role::Student => {
            let student = auth_user.into_user().into_student().unwrap();
            let pref = StudentPreferences::load_for(&student, &db)?;

            let partner = pref.partner.as_ref()
                .map_or(Ok(None), |name| User::load_by_username(name, &db))?
                .and_then(|u| u.into_student().ok())
                .filter(|s| s.id() != student.id());

            Page::empty()
                .with_title(dict.overview_title())
                .add_nav_items(nav_items(locale))
                .with_active_nav_route("/prep")
                .with_content(html::student_overview(
                    locale,
                    &pref,
                    &partner,
                ))
        }

        // ===== Tutor or admin ===============================================
        Role::Tutor | Role::Admin => {
            let stats = {
                use diesel::prelude::*;
                use diesel::expression::sql;
                use db::schema::{timeslot_ratings, users};

                let conn = &*db.conn()?;

                let num_students = users::table
                    .filter(sql("role = 'student'"))
                    .count()
                    .get_result::<i64>(conn)?;

                let num_students_with_slots = users::table
                    .inner_join(timeslot_ratings::table)
                    .filter(sql("rating <> 'bad' AND role = 'student'"))
                    .select(sql("count(distinct user_id) as count"))
                    .get_result::<i64>(conn)?;


                let avg_good_rating_per_student = sql("
                    select cast(avg(count) as float) from (
                        select count(*) as count, user_id
                        from timeslot_ratings
                        inner join users
                            on users.id = user_id
                        where rating = 'good' and role = 'student'
                        group by user_id
                    ) as counts
                ").get_result::<f64>(conn)?;

                let avg_ok_rating_per_student = sql("
                    select cast(avg(count) as float) from (
                        select count(*) as count, user_id
                        from timeslot_ratings
                        inner join users
                            on users.id = user_id
                        where rating <> 'bad' and role = 'student'
                        group by user_id
                    ) as counts
                ").get_result::<f64>(conn)?;

                html::TutorAdminStats {
                    num_students: num_students as u64,
                    num_students_with_slots: num_students_with_slots as u64,
                    avg_good_rating_per_student,
                    avg_ok_rating_per_student,
                }
            };

            let content = html::tutor_admin_overview(
                locale,
                auth_user.is_tutor(),
                stats,
            );

            Page::empty()
                .with_title(dict.overview_title())
                .add_nav_items(nav_items(locale))
                .with_active_nav_route("/prep")
                .with_content(content)
        }
    }.make_ok()
}

#[post("/prep_student_settings", data = "<form>")]
pub fn set_general_settings(
    auth_user: AuthUser,
    form: Form<GeneralStudentSettings>,
    db: State<Db>,
    _state: PreparationState,
    locale: Locale,
) -> Result<Flash<Redirect>> {
    fn err<S: AsRef<str>>(msg: S) -> Result<Flash<Redirect>> {
        Ok(Flash::error(Redirect::to("/prep"), msg))
    }

    let dict = dict::new(locale).prep;

    // The auth_user needs to be a student. Tutors and admins should not be
    // forwarded to this route.
    let student = match auth_user.into_user().into_student() {
        Ok(s) => s,
        Err(_) => {
            return err(bad_request(locale));
        }
    };

    let mut pref = StudentPreferences::load_for(&student, &db)?;
    let form = form.into_inner();

    // Set partner
    match form.partner.as_ref() {
        "random" => {
            pref.partner = None;
        }
        "chosen" => {
            if let Some(id) = form.partner_id {
                match User::load_by_username(&id, &db)? {
                    Some(ref u) if u.is_student() => {
                        pref.partner = Some(id);
                    }
                    Some(ref u) => {
                        return Ok(Flash::error(
                            Redirect::to("/prep"),
                            dict.flash_err_partner_not_a_student(u.username()),
                        ));
                    }
                    None => {
                        return Ok(Flash::error(
                            Redirect::to("/prep"),
                            dict.flash_err_user_not_found(),
                        ));
                    }
                }
            } else {
                return err(bad_request(locale));
            }
        }
        _ => return err(bad_request(locale)),
    }

    // Set preferred language
    match form.language.as_ref() {
        "de" => pref.prefers_english = false,
        "en" => pref.prefers_english = true,
        _ => return err(bad_request(locale)),
    }

    // Finally, store the changes in the database.
    pref.update(&db)?;

    Ok(Flash::success(Redirect::to("/prep"), dict.flash_success_storing_preferences()))
}

#[derive(Debug, Clone, FromForm)]
pub struct GeneralStudentSettings {
    partner: String,
    partner_id: Option<String>,
    language: String,
}

#[get("/prep/timeslots")]
pub fn timeslots(
    auth_user: AuthUser,
    locale: Locale,
    db: State<Db>,
    _state: PreparationState,
) -> Result<Page> {
    let dict = dict::new(locale).prep;

    // Load all ratings of the user.
    let ratings = TimeSlotRating::load_all_of_user(&auth_user, &db)?;

    match auth_user.role() {
        Role::Student | Role::Tutor => {
            let (explanation, min_good, min_ok) = match auth_user.role() {
                Role::Student => (
                    dict.timeslots_student_explanation(),
                    config::MIN_GOOD_SLOTS_STUDENT,
                    config::MIN_OK_SLOTS_STUDENT,
                ),
                Role::Tutor => (
                    dict.timeslots_tutor_explanation(),
                    config::MIN_GOOD_SLOTS_TUTOR,
                    config::MIN_OK_SLOTS_TUTOR,
                ),
                _ => unreachable!(),
            };

            let content = html::timeslots(
                &explanation,
                min_good,
                min_ok,
                &ratings,
                locale,
            );

            Page::empty()
                .with_title(dict.timeslots_title())
                .add_nav_items(nav_items(locale))
                .with_active_nav_route("/prep/timeslots")
                .with_content(content)
                .make_ok()
        }
        Role::Admin => {
            Page::unimplemented().make_ok()
        }
    }
}

/// Stores a list of (timeslot_id, rating).
#[derive(Debug)]
pub struct TimeSlotForm {
    slots: Vec<(i16, Rating)>,
}

impl<'f> FromForm<'f> for TimeSlotForm {
    type Error = TimeSlotFormError;
    fn from_form(items: &mut FormItems<'f>, _: bool) -> StdResult<Self, Self::Error> {
        let slots = items.into_iter().map(|(key, value)| {
            // The keys come in the form `slot-34` and we want this number.
            if !key.starts_with("slot-") {
                return Err(TimeSlotFormError::InvalidId);
            }

            let id = match key[5..].parse() {
                Err(_) => return Err(TimeSlotFormError::InvalidId),
                Ok(id) => id,
            };

            // The value should only be one of those three values.
            let rating = match value.as_str() {
                "good" => Rating::Good,
                "tolerable" => Rating::Tolerable,
                "bad" => Rating::Bad,
                _ => return Err(TimeSlotFormError::InvalidRating),
            };

            Ok((id, rating))
        }).collect::<StdResult<Vec<_>, _>>()?;

        Ok(Self { slots })
    }
}

#[derive(Debug)]
pub enum TimeSlotFormError {
    InvalidRating,
    InvalidId,
}


#[post("/prep/update_timeslots", data = "<form>")]
fn update_timeslots(
    auth_user: AuthUser,
    form: Form<TimeSlotForm>,
    locale: Locale,
    db: State<Db>,
    _state: PreparationState,
) -> Result<Flash<Redirect>> {
    let form = form.into_inner();
    TimeSlotRating::update_all(&auth_user, &form.slots, &db)?;

    Ok(Flash::success(
        Redirect::to("/prep/timeslots"),
        dict::new(locale).prep.flash_success_storing_timeslot_ratings(),
    ))
}
