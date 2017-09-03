use maud::Markup;
use rocket::config::Config;
use rocket::State;

use db::Db;
use dict::Locale;
use super::html;
use template::Page;
use user::AuthAdmin;


#[get("/admin_panel")]
pub fn index(
    admin: AuthAdmin,
    locale: Locale,
    _db: State<Db>,
    config: State<Config>,
) -> Markup {
    // TODO: use real numbers
    let stats = html::Stats {
        num_admins: 0,
        num_tutors: 0,
        num_students: 0,
    };

    Page::empty()
        .with_title("Admin Panel")
        .with_auth_user(&admin)
        .with_content(html::index(locale, &stats, &config))
        .render()
}
