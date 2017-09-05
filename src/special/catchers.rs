use rocket::response::{Flash, Redirect};
use rocket::Request;
use rocket::http::Cookie;

use config;
use dict::{self, Locale};
use errors::*;
use template::{FlashBubble, Page};
use user::AuthUser;


/// Catcher for 403 Forbidden.
///
/// Redirect to `/login` if there is no login present. Show a big red error
/// otherwise.
#[error(403)]
fn unauthorized(req: &Request) -> StdResult<Flash<Redirect>, Page> {
    if req.guard::<AuthUser>().is_success() {
        // In this case a login IS present, but it lacks the permissions to do
        // something. In this case it doesn't make sense to forward to the
        // login page. We will show an error instead.
        let locale = req.guard::<Locale>().unwrap();

        Page::empty()
            .add_flashes(vec![
                FlashBubble::error(dict::new(locale).forbidden_flash()),
            ])
            .make_err()
    } else {
        // In this case, there is no login present. We will forward to the
        // login page.
        let uri = req.uri().as_str().to_owned();
        let cookie = Cookie::build(config::INITIAL_REQ_COOKIE_NAME, uri)
            .path("/")
            .finish();
        req.cookies().add(cookie);

        Ok(Flash::error(
            Redirect::to("/login"),
            "Du musst eingeloggt sein, um die angefragte Seite zu sehen."
        ))
    }
}
