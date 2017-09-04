use maud::{html, Markup};
use rocket::config::Config;

use config;
use dict::{self, Locale};
use state::CurrentAppState;


pub fn index(locale: Locale, stats: &Stats, config: &Config) -> Markup {
    let dict = dict::new(locale).admin_panel;

    html! {
        h1 (dict.title())
        a href="/admin_panel/state" (dict.state_title())

        h2 (dict.statistics_headline())
        ul {
            li { (dict.num_admins()) ": " (stats.num_admins) }
            li { (dict.num_tutors()) ": " (stats.num_tutors) }
            li { (dict.num_students()) ": " (stats.num_students) }
        }

        h2 (dict.config_headline())
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

pub fn state(locale: Locale, app_state: &CurrentAppState) -> Markup {
    let dict = dict::new(locale).admin_panel;

    html! {
        h1 (dict.state_title())

        ul {
            li {
                b (dict.current_state())
                ": "
                (format!("{:?}", app_state.state))
            }
            li {
                b (dict.state_reason())
                ": "
                (
                    app_state.reason
                        .as_ref()
                        .map(|s| html! { "\"" (s) "\"" })
                        .unwrap_or(html! { i (dict.no_reason()) })
                )
            }
            li {
                b (dict.next_state_switch())
                ": "
                (
                    app_state.next_state_switch
                        .map(|dt| html! { (dt) })
                        .unwrap_or(html! { (dict.no_state_switch_estimate()) })
                )
            }
        }
    }
}
