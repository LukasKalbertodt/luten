use rocket::State;
use maud::html;

use user::AuthUser;
use db::Db;
use template::{Flash, Page};


#[get("/settings")]
pub fn settings(_auth_user: AuthUser, _db: State<Db>) -> Page {
    // TODO: implement

    Page::empty()
        .add_flashes(vec![
            Flash::warning(html! { "Settings-page is not implemented yet!" })
        ])
        .with_content(html! {
            "ToDo: Settings"
        })
}
