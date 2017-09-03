#![feature(plugin, custom_derive, proc_macro)]
#![plugin(rocket_codegen)]
#![allow(unused_doc_comment)]
// ^ Otherwise, `error_chain!` and `quick_error!` generate a warning :/

extern crate accept_language;
extern crate chrono;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;
#[macro_use] extern crate error_chain;
extern crate hex;
#[macro_use] extern crate lazy_static;
extern crate maud;
extern crate mauzi;
extern crate option_filter;
extern crate palette;
extern crate pwhash;
#[macro_use] extern crate quick_error;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rand;
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
pub mod util;

pub mod admin_panel;
pub mod config;
pub mod db;
pub mod dict;
pub mod errors;
pub mod login;
pub mod special_routes;
pub mod template;
pub mod user;


pub mod dummy {
    use rocket::State;
    use maud::{html, Markup};

    use user::{AuthUser, User};
    use db::Db;
    use template::Page;

    #[get("/")]
    fn index(auth_user: AuthUser, db: State<Db>) -> Markup {
        let user = User::from_username("anna", &db);

        Page::empty()
            .with_auth_user(&auth_user)
            .with_content(html! {
                "sup " (auth_user.username())
                br;
                (format!("{:?}", user))
                br;
                a href="/dummy" "Dummy page"
                br;
                a href="/logout" "Logout"
            })
            .render()
    }

    #[get("/dummy")]
    fn dummy(auth_user: AuthUser) -> Markup {
        Page::empty()
            .with_auth_user(&auth_user)
            .with_title("Dummy")
            .with_content(html! {
                h2 "This is a dummy page"
                a href="/" "Back"
            })
            .render()
    }
}

pub fn start_server() {
    use db::Db;
    use rocket::fairing::AdHoc;

    rocket::ignite()
        .manage(Db::open_connection())
        .attach(AdHoc::on_attach(|rocket| {
            // Here we insert the Rocket configuration as managed state to
            // retrieve it later.
            let config = rocket.config().clone();
            Ok(rocket.manage(config))
        }))
        .mount("/", routes![
            admin_panel::routes::index,

            dummy::index,
            dummy::dummy,

            special_routes::static_files,

            login::routes::login_form,
            login::routes::validate_data,
            login::routes::logout,

            user::routes::settings,
        ])
        .catch(errors![
            special_routes::unauthorized,
        ])
        .launch();
}
