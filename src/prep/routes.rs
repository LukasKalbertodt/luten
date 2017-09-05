use rocket::State;

use super::html;
use db::Db;
use dict::{self, Locale};
use errors::*;
use state::PreparationState;
use template::{NavItem, Page};
use user::{AuthUser, Role};


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
    _db: State<Db>,
    _state: PreparationState,
) -> Result<Page> {
    let dict = dict::new(locale).prep;

    match auth_user.role() {
        // ===== Student ======================================================
        Role::Student => {
            Page::empty()
                .with_title(dict.overview_title())
                .add_nav_items(nav_items(locale))
                .with_active_nav_route("/prep")
                .with_content(html::student_overview(locale))
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
