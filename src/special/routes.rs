use std::path::{Path, PathBuf};

use rocket::response::{NamedFile, Redirect};
use rocket::State;

use db::Db;
use errors::*;
use state::{AppState, CurrentAppState};
use template::Page;
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
