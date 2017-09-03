use maud::{html, Markup};
use rocket::config::Config;

use config;
use dict::{self, Locale};


pub fn index(locale: Locale, stats: &Stats, config: &Config) -> Markup {
    let dict = dict::new(locale).admin_panel;

    html! {
        h1 (dict.title())
        h2 (dict.statistics_title())
        ul {
            li { (dict.num_admins()) ": " (stats.num_admins) }
            li { (dict.num_tutors()) ": " (stats.num_tutors) }
            li { (dict.num_students()) ": " (stats.num_students) }
        }

        h2 (dict.config_title())
        ul {
            li tt { "WEBSITE_TITLE: " (config::WEBSITE_TITLE) }
            li tt { "SESSION_COOKIE_NAME: " (config::SESSION_COOKIE_NAME) }
            li tt { "SESSION_ID_LEN: " (config::SESSION_ID_LEN) }
            li tt { "INITIAL_REQ_COOKIE_NAME: " (config::INITIAL_REQ_COOKIE_NAME) }
        }
        h3 "Rocket:"
        code (format!("{:#?}", config))
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Stats {
    pub num_admins: u64,
    pub num_tutors: u64,
    pub num_students: u64,
}
