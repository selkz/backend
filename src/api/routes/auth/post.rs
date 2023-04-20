use axum::{http::StatusCode, Json};
use hmac::{Hmac, Mac};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use sha256;

use crate::db::{Database, NewUser};

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
    email: String
}

pub async fn handler(Json(payload): Json<Input>, mut db: Database) -> (StatusCode, Json<Return>) {

    let pwd_hash = sha256::digest(payload.password);
    let token = token_generator::TokenGenerator::new(0, usize::MAX).gen();
    // jwt gen

    let new_user = NewUser{
        id: Uuid::new_v4().to_string(),
        email: payload.email,
        username: payload.username,
        password: pwd_hash,
        session_token: token
    };

    let Ok(_) = db.register_user(&new_user).await else {
        return (StatusCode::BAD_REQUEST, Json(Return{
            message: "Failed to create user".into(),
            user: None,
        }));
    };


    let user = ReturnUser{
        id: new_user.id,
        username: new_user.username,
        email: new_user.email,
    };

    (StatusCode::OK, Json(Return{
        message: "User created".into(),
        user: Some(user)
    }))
}