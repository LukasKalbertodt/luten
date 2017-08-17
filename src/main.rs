#![feature(plugin, custom_derive, proc_macro)]
#![plugin(rocket_codegen)]

extern crate chrono;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;
extern crate hex;
extern crate maud;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rand;
extern crate rocket;
extern crate rocket_contrib;


mod config;
mod db;
mod login;
mod special_routes;
mod template;
mod user;

mod dummy {
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

fn main() {
    use db::Db;

    rocket::ignite()
        .manage(Db::open_connection())
        .mount("/", routes![
            dummy::index,
            dummy::dummy,

            special_routes::static_files,

            login::routes::login_form,
            login::routes::validate_data,
            login::routes::logout,
        ])
        .catch(errors![
            special_routes::unauthorized,
        ])
        .launch();
}
