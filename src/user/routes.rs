use rocket::State;

use user::AuthUser;
use db::Db;
use template::Page;


#[get("/settings")]
pub fn settings(_auth_user: AuthUser, _db: State<Db>) -> Page {
    Page::unimplemented()
}
