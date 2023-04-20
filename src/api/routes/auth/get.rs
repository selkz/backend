use axum::{http::StatusCode, Json};
use hmac::{Hmac, Mac};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use sha256;

use crate::{db::{Database, User}, api::Api};

#[derive(Deserialize)]
pub struct Input {
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
    token: String,
}

pub async fn handler(Json(payload): Json<Input>, mut api: Api) -> (StatusCode, Json<Return>) {

    let pwd_hash = sha256::digest(payload.password);

    let db_user = match api.db.get_user(payload.email).await {
        Ok(v) => v,
        Err(e) => {
            return (StatusCode::BAD_REQUEST, Json(Return{
                message: format!("Email not found: {e}"),
                user: None,
            }));
        }
    };
    if db_user.password != pwd_hash {
        return (StatusCode::UNAUTHORIZED, Json(Return{
            message: format!("Wrong password"),
            user: None,
        }));
    }

    let user = ReturnUser{
        id: db_user.id,
        username: db_user.username,
        email: db_user.email,
        token: db_user.session_token,
    };

    (StatusCode::OK, Json(Return{
        message: "User found".into(),
        user: Some(user)
    }))
}