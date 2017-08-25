use maud::Markup;
use rocket::config::Environment;
use rocket::http::{Cookies, Status};
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, Redirect};
use rocket::State;

use config;
use db::Db;
use errors::*;
use super::{html, login};
use template::Page;
use user::AuthUser;


/// Shows a login form.
///
/// If this page is visited while logged in, the user is redirected to `/` to
/// avoid potential confusion.
#[get("/login")]
fn login_form(
    auth_user: Option<AuthUser>,
    flash: Option<FlashMessage>,
    env: State<Environment>,
) -> StdResult<Markup, Redirect> {
    match auth_user {
        // If the user is already logged in, we just forward them to the index
        // page. They shouldn't be able to see the login form. It's confusing.
        Some(_) => Err(Redirect::to("/")),

        // Otherwise we show the login form.
        None => {
            // If we're in production, filter out the providers that are tagged
            // `dev_only`.
            // TODO: we want to give developers the ability to use `dev_only`
            // providers in production. Maybe with a magic `i_am_a_dev` cookie
            // or something stupid like that.
            let providers = config::LOGIN_PROVIDERS.iter()
                .filter(|prov| !(prov.dev_only && *env.inner() == Environment::Production))
                .collect::<Vec<_>>();

            let page = Page::empty()
                .with_title("Login")
                .with_content(html::login_page(&providers))
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

    // Find the login provider the user chose. If there is no such provider,
    // we respond with "400 Bad Request".
    let login_provider = config::LOGIN_PROVIDERS.iter()
        .find(|prov| prov.id == form.login_provider)
        .ok_or(ErrorKind::BadHttp(Status::BadRequest))?;

    let res = login(&form.id, &form.secret, &*login_provider.imp, cookies, &db);
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
fn logout(auth_user: Option<AuthUser>, cookies: Cookies, db: State<Db>) -> Result<Flash<Redirect>> {
    if let Some(auth_user) = auth_user {
        auth_user.destroy_session(cookies, &db)?;
    }
    Ok(Flash::success(
        Redirect::to("/login"),
        "Du wurdest erfolgreich ausgeloggt",
    ))
}

#[derive(FromForm)]
struct LoginForm {
    id: String,
    secret: String,
    login_provider: String,
}
