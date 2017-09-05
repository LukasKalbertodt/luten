//! A web app to manage homework assignments.
//!
//! This documentation is written for luten developers and ordinary users
//! will probably not benefit from this.
//!
//!
//! ## Overview
//!
//! The main code of this application lives in a library crate called `luten`.
//! Right now, you are reading the documentation of its root module. This is a
//! library crate to allow access to "model" types from other tools. You can
//! find some helper tools in the `src/bin/` directory.
//!
//! You can find more detailed information on all submodules in the respective
//! documentations.

// Necessary for Rocket & Maud
#![feature(plugin, custom_derive, proc_macro)]
#![plugin(rocket_codegen)]

// Otherwise, `error_chain!` and `quick_error!` generate warnings :/
#![allow(unused_doc_comment)]

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


pub mod admin_panel;
pub mod config;
pub mod db;
pub mod dict;
pub mod errors;
pub mod login;
pub mod prep;
pub mod special;
pub mod state;
pub mod template;
pub mod user;


/// Starts a Rocket server with all routes mounted.
///
/// This is basically the `main()` function of this crate. To actually start
/// the server, there is a small executable `src/bin/start_server.rs`. You can
/// execute it via `cargo run --bin start_server`.
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

            login::routes::login_form,
            login::routes::validate_data,
            login::routes::logout,

            prep::routes::overview,

            special::routes::static_files,
            special::routes::index,

            user::routes::settings,
        ])
        .catch(errors![
            special::catchers::unauthorized,
        ])
        .launch();
}
