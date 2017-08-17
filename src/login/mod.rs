// use model::AuthUser;
// use context::Context;
// use db::Db;



pub mod routes;
mod html;




// /// The main login page showing a login form.
// ///
// /// We might want to embed a smaller form into another route (the index page
// /// for example), but this route will still be available.
// #[get("/login", rank = 3)]
// fn without_login(flash: Option<FlashMessage>) -> Template {
//     let context = Context {
//         flash: flash.map(|f| f.into()),
//         .. Context::empty()
//     };
//     Template::render("login", &context)
// }

// /// Handler in case the `/login` page is access although the user is already
// /// logged in. We just redirect to the index page.
// #[get("/login")]
// fn with_login(_user: AuthUser) -> Redirect {
//     // TODO: GitHub uses the 302 status code to redirect, but the `to()` method
//     // uses the code 303. The rocket docs say 303 is preferred over 302, but
//     // we should look for more information on this.
//     Redirect::to("/")
// }

// /// Handles post data from a login action.
// #[post("/login", data = "<form>")]
// fn validate_data(
//     cookies: &Cookies,
//     form: Form<LoginForm>,
//     db: State<Db>,
// ) -> Result<Redirect, Flash<Redirect>> {
//     let form = form.into_inner();
//     match AuthUser::login(&form.id, &form.password, &db) {
//         Ok(mut user) => {
//             user.create_session(&cookies, &db);
//             Ok(Redirect::to("/"))
//         }
//         Err(e) => {
//             Err(Flash::error(Redirect::to("/login"), e.description()))
//         }
//     }
// }

// /// Handler to logout the user. If there is no login present, nothing happens.
// #[get("/logout")]
// fn logout(cookies: &Cookies, user: Option<AuthUser>, db: State<Db>) -> Redirect {
//     if let Some(user) = user {
//         user.end_session(&cookies, &db);
//     }
//     Redirect::to("/")
// }

// #[derive(FromForm)]
// struct LoginForm {
//     id: String,
//     password: String,
// }
