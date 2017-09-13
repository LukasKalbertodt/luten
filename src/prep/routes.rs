use rocket::State;
use rocket::response::{Flash, Redirect};
use rocket::request::{Form, FormItems, FromForm};
use option_filter::OptionFilterExt;

use super::{html, StudentPreferences};
use db::Db;
use dict::{self, Locale};
use errors::*;
use state::PreparationState;
use template::{NavItem, Page};
use user::{AuthUser, Role, User};
use timeslot::{Rating, TimeSlot};


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

        // ===== Tutor ========================================================
        Role::Tutor => {
            Page::unimplemented()
        }

        // ===== Admin ========================================================
        Role::Admin => {
            Page::unimplemented()
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
                            dict.flash_partner_not_a_student(u.username()),
                        ));
                    }
                    None => {
                        return Ok(Flash::error(
                            Redirect::to("/prep"),
                            dict.flash_user_not_found(),
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
    let _dict = dict::new(locale).prep;

    let timeslots = {
        let mut timeslots = TimeSlot::load_all(&db)?;
        timeslots.sort();
        timeslots
    };

    match auth_user.role() {
        Role::Student => {

            let content = html::student_timeslots(&timeslots, locale);

            Page::empty()
                .with_content(content)
                .make_ok()
        }
        Role::Tutor => {
            Page::unimplemented().make_ok()
        }
        Role::Admin => {
            Page::unimplemented().make_ok()
        }
    }
}

#[derive(Debug)]
pub struct TimeSlotForm {
    slots: Vec<(i64, Rating)>,
}

impl<'f> FromForm<'f> for TimeSlotForm {
    type Error = TimeSlotFormError;
    fn from_form(items: &mut FormItems<'f>, _: bool) -> StdResult<Self, Self::Error> {
        let slots = items.into_iter().map(|(key, value)| {
            if !key.starts_with("slot-") {
                return Err(TimeSlotFormError::InvalidId);
            }

            let id = match key[5..].parse() {
                Err(_) => return Err(TimeSlotFormError::InvalidId),
                Ok(id) => id,
            };

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
    _auth_user: AuthUser,
    form: Form<TimeSlotForm>,
    _locale: Locale,
    _db: State<Db>,
    _state: PreparationState,
) -> Result<Flash<Redirect>> {
    println!("{:#?}", form);

    Ok(Flash::success(
        Redirect::to("/prep/timeslots"),
        "Well well",
    ))
}
