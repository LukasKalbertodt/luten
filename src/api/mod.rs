use std::ops::Try;

use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{Response, Responder};
use rocket_contrib::Json;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;


pub mod routes;



#[derive(Debug, Clone)]
pub enum ApiResponse<T> {
    Ok(T),
    BadRequest(String),
    InternalServerError,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn is_ok(&self) -> bool {
        if let ApiResponse::Ok(_) = *self { true } else { false }
    }
}

impl<T: Serialize> Try for ApiResponse<T> {
    type Ok = T;
    type Error = ();

    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        match self {
            ApiResponse::Ok(payload) => Ok(payload),
            _ => Err(()),
        }
    }

    fn from_error(_: Self::Error) -> Self {
        ApiResponse::InternalServerError
    }

    fn from_ok(v: Self::Ok) -> Self {
        ApiResponse::Ok(v)
    }
}

impl<T: Serialize> Serialize for ApiResponse<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_struct("ApiResponse", 3)?;
        s.serialize_field("ok", &self.is_ok())?;

        match *self {
            ApiResponse::Ok(ref payload) => {
                s.serialize_field("data", payload)?;
            }
            ApiResponse::BadRequest(ref e) => {
                s.serialize_field("err", e)?;
            }
            ApiResponse::InternalServerError => {}
        }

        s.end()
    }
}

impl<'r, T: Serialize> Responder<'r> for ApiResponse<T> {
    fn respond_to(self, req: &Request) -> Result<Response<'r>, Status> {
        let status = match self {
            ApiResponse::Ok(_) => Status::Ok,
            ApiResponse::BadRequest(_) => Status::BadRequest,
            ApiResponse::InternalServerError => Status::InternalServerError,
        };

        Json(self)
            .respond_to(req)
            .map(|mut resp| {
                resp.set_status(status);
                resp
            })
    }
}
