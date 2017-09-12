use maud::{html, Markup};
use rocket::config::Config;

use config;
use dict::{self, Locale};
use state::CurrentAppState;
use timeslot::{DayOfWeek, TimeSlot};


pub fn index(locale: Locale, stats: &Stats, config: &Config) -> Markup {
    let dict = dict::new(locale).admin_panel;

    html! {
        h1 (dict.title())
        ul {
            li a href="/admin_panel/state" (dict.state_title())
            li a href="/admin_panel/timeslots" (dict.timeslots_title())

        }

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
    let root_dict = dict::new(locale);
    let dict = &root_dict.admin_panel;

    html! {
        h1 (dict.state_title())

        h2 (dict.current_state())

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

        h2 (dict.change_state())

        form action="/admin_panel/state" method="post" {
            select class="c-field" name="state" {
                option value="preparation" "Preparation"
                option value="running" "Running"
                option value="frozen" "Frozen"
            }

            div class="o-form-element" {
              label class="c-label" for="reason" (dict.state_reason())
              input id="reason" name="reason" class="c-field";
            }

            input
                class="c-button c-button--success"
                type="submit"
                value=(root_dict.save_form()) {}
        }
    }
}

pub fn timeslots(timeslots: &[TimeSlot], locale: Locale) -> Markup {
    let root_dict = dict::new(locale);
    let dict = &root_dict.admin_panel;

    html! {
        h1 (dict.timeslots_title())

        table class="c-table timeslot-table" {
            thead class="c-table__head" {
                tr class="c-table__row c-table__row--heading" {
                    th class="c-table__cell" "Day"
                    th class="c-table__cell" "Time"
                    th class="c-table__cell" {}
                }
            }

            tbody class="c-table__body" {
                @for timeslot in timeslots {
                    tr class="c-table__row" {
                        td class="c-table__cell" (timeslot.day())
                        td class="c-table__cell" (timeslot.time())
                        td class="c-table__cell" {
                            form {
                                input
                                    type="submit"
                                    class="c-button c-button--error"
                                    value="Delete";
                            }
                        }
                    }
                }

                tr class="c-table__row" {
                    form action="/admin_panel/timeslots" method="post" {
                        td class="c-table__cell" {
                            select id="new_timeslot_day" {
                                @for name in DayOfWeek::all_names() {
                                    option (name)
                                }
                            }
                        }
                        td class="c-table__cell" {
                            input type="text";
                        }
                        td class="c-table__cell" {
                            input
                                type="submit"
                                class="c-button c-button--success"
                                value="Add";
                        }
                    }
                }
            }
        }
    }
}
