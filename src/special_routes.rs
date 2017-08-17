use std::path::{Path, PathBuf};

use rocket::response::{NamedFile, Redirect};
use rocket::Request;
use rocket::http::Cookie;

use config;

/// Route to serve static file requests from the `static/` directory.
#[get("/static/<path..>")]
pub fn static_files(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}


#[error(401)]
fn unauthorized(req: &Request) -> Redirect {
    let uri = req.uri().as_str().to_owned();
    let cookie = Cookie::build(config::INITIAL_REQ_COOKIE_NAME, uri)
        .path("/")
        .finish();
    req.cookies().add(cookie);

    Redirect::to("/login")
}
