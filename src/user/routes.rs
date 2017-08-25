use rocket::State;
use maud::{html, Markup};

use user::AuthUser;
use db::Db;
use template::{Flash, Page};


#[get("/settings")]
pub fn settings(auth_user: AuthUser, _db: State<Db>) -> Markup {
    // TODO: implement

    Page::empty()
        .with_auth_user(&auth_user)
        .add_flashes(vec![
            Flash::warning(html! { "Settings-page is not implemented yet!" })
        ])
        .with_content(html! {
            "ToDo: Settings"
        })
        .render()
}
