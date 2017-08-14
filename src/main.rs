#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;


pub mod model;
pub mod route;

fn main() {
    rocket::ignite()
        .mount("/", routes![
            route::dummy,
        ])
        .launch();
}
