use axum::{http::StatusCode, Json};
use serde::Serialize;

pub mod user;
pub mod auth;

#[derive(Serialize)]
pub struct Return {
    message: String
}

pub async fn root() -> (StatusCode, Json<Return>) {
    (StatusCode::OK, Json(Return{
        message: "Hewo world!".into()
    }))
}