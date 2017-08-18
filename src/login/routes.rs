use maud::{Markup};
use rocket::response::{Flash, Redirect};
use rocket::http::Cookies;
use rocket::request::Form;
use rocket::State;

use db::Db;
use super::{html, login};
use template::Page;
use user::AuthUser;
use password;


#[get("/login")]
fn login_form(auth_user: Option<AuthUser>) -> Result<Markup, Redirect> {
    match auth_user {
        // If the user is already logged in, we just forward them to the index
        // page. They shouldn't be able to see the login form. It's confusing.
        Some(_) => Err(Redirect::to("/")),

        // Otherwise we show the login form.
        None => {
            let page = Page::empty()
                .with_title("Login")
                .with_content(html::content())
                .render();

            Ok(page)
        }
    }
}


/// Handles post data from a login action.
#[post("/login", data = "<form>")]
fn validate_data(
    cookies: Cookies,
    form: Form<LoginForm>,
    db: State<Db>,
) -> Result<Redirect, Flash<Redirect>> {
    let form = form.into_inner();
    let login_provider = password::InternalProvider;

    let res = login(&form.id, &form.secret, &login_provider, cookies, &db);
    match res {
        Ok(_) => {
            // TODO: redirect to the original request path
            Ok(Redirect::to("/"))
        }
        Err(e) => {
            // TODO: proper error message
            Err(Flash::error(Redirect::to("/login"), format!("{:?}", e)))
        }
    }
}

/// Handler to logout the user. If there is no login present, nothing happens.
#[get("/logout")]
fn logout(auth_user: Option<AuthUser>, cookies: Cookies, db: State<Db>) -> Redirect {
    if let Some(auth_user) = auth_user {
        auth_user.destroy_session(cookies, &db);
    }
    Redirect::to("/")
}

#[derive(FromForm)]
struct LoginForm {
    id: String,
    secret: String,
}
