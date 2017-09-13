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
//!
//!
//! ## Module structure
//!
//! There are different kinds of top-level modules in this crate:
//!
//! - **Helper/internal modules**: these modules don't contain anything that is
//!   directly shown on the website. Rather, stuff from these modules is used
//!   in other modules. List: [`config`](config/index.html),
//!   [`db`](db/index.html), [`errors`](errors/index.html),
//!   [`state`](state/index.html) and [`template`](template/index.html).
//! - **Route modules**: these modules are directly responsible for what is
//!   visible on the website. They contain the route-handlers. List:
//!   [`admin_panel`](admin_panel/index.html), [`login`](login/index.html),
//!   [`prep`](prep/index.html), [`special`](special/index.html) and
//!   [`user`](user/index.html).
//! - **Dictionary modules**: the root dictionary module is
//!   [`dict`](dict/index.html). Please see its documentation for more
//!   information.
//!
//! The directories of route modules often contain the following files:
//!
//! - `mod.rs`: root of sub-module (crude MVC equivalent: model)
//! - `routes.rs`: definition of all routes (crude MVC equivalent: controller)
//! - `html.rs`: the HTML definition (crude MVC equivalent: view)
//! - `mod.mauzi.rs`: the dictionary definition for this module

// Necessary for Rocket & Maud
#![feature(plugin, custom_derive, proc_macro, try_trait)]
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
extern crate ldap3;
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
extern crate serde;
#[macro_use] extern crate serde_derive;


pub mod api;
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
pub mod timeslot;
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
            admin_panel::routes::change_state,
            admin_panel::routes::timeslots,
            admin_panel::routes::add_timeslot,
            admin_panel::routes::delete_timeslot,

            login::routes::login_form,
            login::routes::validate_data,
            login::routes::logout,

            prep::routes::overview,
            prep::routes::set_general_settings,
            prep::routes::timeslots,
            prep::routes::update_timeslots,

            special::routes::static_files,
            special::routes::scss_files,
            special::routes::index,

            user::routes::settings,
        ])
        .mount("/api", routes![
            api::routes::user::by_username,
        ])
        .catch(errors![
            special::catchers::unauthorized,
        ])
        .launch();
}
