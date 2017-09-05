use accept_language;
use chrono::{DateTime, Utc};
use maud::{html, Markup};
use mauzi::mauzi;
use rocket::Outcome;
use rocket::request::{self, FromRequest, Request};


impl<'a, 'r> FromRequest<'a, 'r> for Locale {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
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
                    "\"" (reason) "\""
                }
            }

            @if let Some(end) = end {
                p {
                    b "Vorraussichtliche Reaktivierung des Systems: "
                    "„" (end) "“"
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
                    "\"" (reason) "\""
                }
            }

            @if let Some(end) = end {
                p {
                    b "Estimated time when the system is unfrozen: "
                    "“" (end) "”"
                }
            }
        }}
    }
}
