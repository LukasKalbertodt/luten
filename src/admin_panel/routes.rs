use std::collections::HashMap;

use diesel::prelude::*;
use maud::Markup;
use rocket::config::Config;
use rocket::State;

use db::Db;
use dict::Locale;
use errors::*;
use super::html;
use template::Page;
use user::{AuthAdmin, Role};


#[get("/admin_panel")]
pub fn index(
    admin: AuthAdmin,
    locale: Locale,
    db: State<Db>,
    config: State<Config>,
) -> Result<Markup> {
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
        .with_auth_user(&admin)
        .with_content(html::index(locale, &stats, &config))
        .render()
        .make_ok()
}
