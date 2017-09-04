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
pub mod prep;
pub mod special_routes;
pub mod state;
pub mod template;
pub mod user;


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
            admin_panel::routes::state,

            special_routes::static_files,
            special_routes::index,

            login::routes::login_form,
            login::routes::validate_data,
            login::routes::logout,

            prep::routes::overview,

            user::routes::settings,
        ])
        .catch(errors![
            special_routes::unauthorized,
        ])
        .launch();
}
