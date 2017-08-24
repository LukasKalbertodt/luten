use maud::{Markup};
use rocket::response::{Flash, Redirect};
use rocket::http::Cookies;
use rocket::request::{FlashMessage, Form};
use rocket::State;

use db::Db;
use errors::*;
use super::{html, login};
use template::Page;
use user::AuthUser;
use login::password;


/// Shows a login form.
///
/// If this page is visited while logged in, the user is redirected to `/` to
/// avoid potential confusion.
#[get("/login")]
fn login_form(
    auth_user: Option<AuthUser>,
    flash: Option<FlashMessage>,
) -> StdResult<Markup, Redirect> {
    match auth_user {
        // If the user is already logged in, we just forward them to the index
        // page. They shouldn't be able to see the login form. It's confusing.
        Some(_) => Err(Redirect::to("/")),

        // Otherwise we show the login form.
        None => {
            let page = Page::empty()
                .with_title("Login")
                .with_content(html::content())
                .add_flashes(flash)
                .render();

            Ok(page)
        }
    }
}


/// Handles post data from a login action.
///
/// Tries to login with the given data. Creates a login-session if the login-
/// attempt was successful.
#[post("/login", data = "<form>")]
fn validate_data(
    cookies: Cookies,
    form: Form<LoginForm>,
    db: State<Db>,
) -> Result<StdResult<Redirect, Flash<Redirect>>> {
    let form = form.into_inner();

    // TODO: this is temporary of course
    let login_provider = password::InternalProvider;

    let res = login(&form.id, &form.secret, &login_provider, cookies, &db);
    match res {
        Ok(_) => {
            // TODO: redirect to the original request path
            Ok(Ok(Redirect::to("/")))
        }
        Err(Error(ErrorKind::LoginError(e), _)) => {
            // TODO: proper error message
            Ok(Err(Flash::error(Redirect::to("/login"), format!("{:?}", e))))
        }
        Err(other) => bail!(other),
    }
}

/// Handler to logout the user. If there is no login present, nothing happens.
#[get("/logout")]
fn logout(auth_user: Option<AuthUser>, cookies: Cookies, db: State<Db>) -> Result<Redirect> {
    if let Some(auth_user) = auth_user {
        auth_user.destroy_session(cookies, &db)?;
    }
    Ok(Redirect::to("/"))
}

#[derive(FromForm)]
struct LoginForm {
    id: String,
    secret: String,
}
