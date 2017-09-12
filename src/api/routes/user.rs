use rocket::http::RawStr;
use rocket::State;

use api::ApiResponse;
use db::Db;
use user::{AuthUser, User};

#[derive(Debug, Serialize)]
pub struct UserResponse {
    name: Option<String>,
    role: String,
}

#[get("/user/by_username/<username>")]
pub fn by_username(
    username: &RawStr,
    db: State<Db>,
    auth_user: Option<AuthUser>,
) -> ApiResponse<Option<UserResponse>> {
    if auth_user.is_none() {
        return ApiResponse::Forbidden;
    }

    match username.percent_decode() {
        Ok(s) => {
            let payload = User::load_by_username(&s, &db)
                .map_err(|_| ())?
                .map(|u| {
                    UserResponse {
                        name: u.name,
                        role: format!("{:?}", u.role),
                    }
                });

            ApiResponse::Ok(payload)
        }
        Err(_) => ApiResponse::BadRequest("Invalid (non-UTF8) username".into()),
    }
}
