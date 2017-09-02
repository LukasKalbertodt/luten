use accept_language;
use mauzi::mauzi;
use rocket::Outcome;
use rocket::request::{self, FromRequest, Request};
use maud::{html, Markup};

mauzi! {
    enum Locale {
        De,
        En,
    }

    mod login;
}

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
