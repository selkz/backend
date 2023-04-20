use axum::{http::StatusCode, Json};
use hmac::{Hmac, Mac};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use sha256;

use crate::{db::{Database, User}, api::Api};

#[derive(Deserialize)]
pub struct Input {
    username: String,
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct Return {
    message: String,
    user: Option<ReturnUser>
}

#[derive(Serialize)]
pub struct ReturnUser {
    id: String,
    username: String,
    email: String,
    token: String
}

pub async fn handler(Json(payload): Json<Input>, mut api: Api) -> (StatusCode, Json<Return>) {

    let id = Uuid::new_v4().to_string();
    let pwd_hash = sha256::digest(payload.password);
    let token = crate::util::gen_token(id.clone());

    let new_user = User{
        id,
        email: payload.email,
        username: payload.username,
        password: pwd_hash,
        session_token: token.clone()
    };

    if let Err(e) = api.db.create_user(&new_user).await {
        return (StatusCode::BAD_REQUEST, Json(Return{
            message: format!("Failed to create user: {e}"),
            user: None,
        }));
    };


    let user = ReturnUser{
        id: new_user.id,
        username: new_user.username,
        email: new_user.email,
        token
    };

    (StatusCode::OK, Json(Return{
        message: "User created".into(),
        user: Some(user)
    }))
}