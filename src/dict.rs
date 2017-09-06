//! This module holds the dictionary used for localization.
//!
//! We want to support multiple languages in our tool, so we have to use some
//! kind of localization. There aren't any stable i18n or l10n crates in the
//! Rust eco-system right now. Thus I (Lukas) wrote my own which I plan to
//! develop alongside with this project. It's called `mauzi` and is very young
//! and bare-bone. Some TODO comments in these dictionary definitions are
//! actually intended for mauzi-development.
//!
//! Anyway, mauzi let's you define dictionaries hosting multiple translations
//! fairly easily. I hope the syntax is more or less self-explanatory.
//!
//! The dictionary definition is spread over multiple files: this file and
//! several `mod.mauzi.rs` files.

use accept_language;
use chrono::{DateTime, Utc};
use maud::{html, Markup};
use mauzi::mauzi;
use rocket::Outcome;
use rocket::request::{self, FromRequest, Request};

// We implement `FromRequest` for the mauzi-generated `Locale` type to easily
// obtain it in any handler.
impl<'a, 'r> FromRequest<'a, 'r> for Locale {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        // TODO: this could be improved... We basically parse the request
        // header and use `En` as default locale.
        let locale = req.headers().get_one("accept-language")
            .and_then(|val| {
                accept_language::intersection(val, vec!["de", "en"]).first().cloned()
            })
            .map(|lang| {
                match lang.as_str() {
                    "de" => Locale::De,
                    "en" => Locale::En,
                    _ => unreachable!(),
                }
            })
            .unwrap_or(Locale::En);

        Outcome::Success(locale)
    }
}

mauzi! {
    enum Locale {
        De,
        En,
    }

    mod admin_panel;
    mod login;
    mod prep;
    mod timeslot;


    // Below are translation units which are somewhat global. Everything that
    // is only used by a specific part of this application should be defined in
    // the corresponding module.

    unit forbidden_flash -> Markup {
        De => { html! {
            "Du hast nicht die notwendigen Rechte, um diese Seite aufzurufen! "
            a href="/" "Zurück zur Startseite."
        }},
        En => { html! {
            "You are lacking the permission to view this page! "
            a href="/" "Back to the index page."
        }}
    }

    unit frozen_flash(reason: Option<&str>, end: Option<DateTime<Utc>>) -> Markup {
        De => { html! {
            p {
                "Das System ist zurzeit deaktiviert. Das bedeutet, dass du "
                "gerade nichts machen kannst. Dieser Zustand wurde "
                "wahrscheinlich von einem Administrator aktiviert und ist "
                "üblicherweise nur temporär."
            }

            @if let Some(reason) = reason {
                p {
                    b "Grund für Deaktivierung: "
                    "„" (reason) "“"
                }
            }

            @if let Some(end) = end {
                p {
                    b "Vorraussichtliche Reaktivierung des Systems: "
                    (end)
                }
            }
        }}

        En => { html! {
            p {
                "This website is frozen right now. This means that you can't "
                "do anything. This state is usually temporary and was "
                "activated by an administrator."
            }

            @if let Some(reason) = reason {
                p {
                    b "Reason: "
                    "“" (reason) "”"
                }
            }

            @if let Some(end) = end {
                p {
                    b "Estimated time when the system is unfrozen: "
                    (end)
                }
            }
        }}
    }


    // Page navigation
    unit nav_account {
        _ => "Account",
    }
    unit nav_settings {
        De => "Einstellungen",
        En => "Settings",
    }
    unit nav_logout {
        De => "Ausloggen",
        En => "Logout",
    }
}
