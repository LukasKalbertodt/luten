use rocket::http::RawStr;
use rocket::State;

use api::ApiResponse;
use db::Db;
use user::{AuthUser, User};



#[get("/user/by_username/<username>")]
pub fn by_username(
    username: &RawStr,
    db: State<Db>,
    _auth_user: AuthUser,
) -> ApiResponse<Option<String>> {
    match username.percent_decode() {
        Ok(s) => {
            let payload = User::load_by_username(&s, &db)
                .map_err(|_| ())?
                .and_then(|u| u.name);

            ApiResponse::Ok(payload)
        }
        Err(_) => ApiResponse::BadRequest("Invalid (non-UTF8) username".into()),
    }
}
