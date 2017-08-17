#![feature(plugin, custom_derive, proc_macro)]
#![plugin(rocket_codegen)]

extern crate maud;
extern crate rocket;
extern crate rocket_contrib;


mod config;
mod login;
mod special_routes;
mod template;
mod user;

mod dummy {
    use maud::{html, Markup};

    use user::AuthUser;
    use template::Page;

    #[get("/")]
    fn index(auth_user: AuthUser) -> Markup {
        Page::empty()
        .with_auth_user(&auth_user)
        .with_content(html! {
            "sup " (auth_user.username())
        })
        .render()
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![
            dummy::index,

            special_routes::static_files,

            login::routes::without_user,
        ])
        .catch(errors![
            special_routes::unauthorized,
        ])
        .launch();
}
