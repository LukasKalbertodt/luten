use std::path::{Path, PathBuf};

use maud::html;
use rocket::response::{Flash, NamedFile, Redirect};
use rocket::{Request, State};
use rocket::http::Cookie;

use config;
use db::Db;
use dict::{self, Locale};
use errors::*;
use state::{AppState, CurrentAppState};
use template::{Flash as OurFlash, Page};
use user::{AuthUser, Role};


/// The index page.
///
/// This handler will always redirect instead of generating a response itself
/// (except in the error case).
#[get("/")]
pub fn index(auth_user: AuthUser, db: State<Db>) -> Result<StdResult<Redirect, Page>> {
    let app_state = CurrentAppState::load(&db)?;

    // Redirect to the correct route depending on user role and app state.
    match (auth_user.role(), app_state.state) {
        // Preparation state
        (Role::Student, AppState::Preparation) => Ok(Redirect::to("/prep")),

        // Frozen state: admins are redirected to the admin panel, all others
        // see a empty page with a flash bubble talking about the state.
        (Role::Admin, AppState::Frozen) => Ok(Redirect::to("/admin_panel")),
        (_, AppState::Frozen) => Err(Page::empty()),

        _ => Err(Page::unimplemented()),
    }.make_ok()
}

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
fn unauthorized(req: &Request) -> StdResult<Flash<Redirect>, Page> {
    if req.guard::<AuthUser>().is_success() {
        // In this case a login IS present, but it lacks the permissions to do
        // something. In this case it doesn't make sense to forward to the
        // login page. We will show an error instead.
        let locale = req.guard::<Locale>().unwrap();

        let page = Page::empty()
            .add_flashes(vec![
                OurFlash::error(html! {
                    (dict::new(locale).forbidden_flash())
                }),
            ]);

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
