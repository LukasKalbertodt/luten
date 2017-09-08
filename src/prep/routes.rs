use rocket::State;
use rocket::response::{Flash, Redirect};
use rocket::request::Form;
use option_filter::OptionFilterExt;

use super::{html, StudentPreferences};
use db::Db;
use dict::{self, Locale};
use errors::*;
use state::PreparationState;
use template::{NavItem, Page};
use user::{AuthUser, Role, User};


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
                // TODO: validate the id exists? Somehow?
                pref.partner = Some(id);
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
