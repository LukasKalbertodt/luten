use std::path::{Path, PathBuf};

use maud::{html, Markup};
use rocket::response::{Flash, NamedFile, Redirect};
use rocket::{Outcome, Request};
use rocket::http::Cookie;

use config;
use dict::{self, Locale};
use errors::*;
use template::{Flash as OurFlash, Page};
use user::AuthUser;

/// Route to serve static file requests from the `static/` directory.
///
/// Thanks to Rocket, this is *path traversal attack* save.
#[get("/static/<path..>")]
pub fn static_files(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}


/// Catcher for 403 Forbidden.
///
/// Redirect to `/login` if there is no login present. Show a big red error
/// otherwise.
#[error(403)]
fn unauthorized(req: &Request) -> StdResult<Flash<Redirect>, Markup> {
    if let Outcome::Success(auth_user) = req.guard::<AuthUser>() {
        // In this case a login IS present, but it lacks the permissions to do
        // something. In this case it doesn't make sense to forward to the
        // login page. We will show an error instead.
        let locale = req.guard::<Locale>().unwrap();

        let page = Page::empty()
            .with_auth_user(&auth_user)
            .add_flashes(vec![
                OurFlash::error(html! {
                    (dict::new(locale).forbidden_flash())
                }),
            ])
            .render();

        Err(page)
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
