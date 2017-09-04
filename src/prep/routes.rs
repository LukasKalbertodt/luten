use maud::{html, Markup};
use rocket::State;

use super::html;
use db::Db;
use dict::{self, Locale};
use errors::*;
use template::{NavItem, Page};
use user::{AuthUser, Role};


#[get("/prep")]
fn overview(
    auth_user: AuthUser,
    locale: Locale,
    _db: State<Db>,
    // TODO: use `StatePreparation` guard
) -> Result<Markup> {
    let dict = dict::new(locale).prep;

    match auth_user.role() {
        // ===== Student ======================================================
        Role::Student => {
            Page::empty()
                .with_auth_user(&auth_user)
                .with_title(dict.overview_title())
                .add_nav_items(vec![
                    NavItem::new(dict.nav_overview_title(), "/prep"),
                    NavItem::new(dict.nav_timeslots_title(), "/prep/timeslots"),
                ])
                .with_content(html::student_overview(locale))
                .render()
        }

        // ===== Tutor ========================================================
        Role::Tutor => {
            Page::error(html! { "unimplemented" })
                .with_auth_user(&auth_user)
                .render()
        }

        // ===== Admin ========================================================
        Role::Admin => {
            Page::error(html! { "unimplemented" })
                .with_auth_user(&auth_user)
                .render()
        }
    }.make_ok()

}
