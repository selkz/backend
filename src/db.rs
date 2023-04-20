use std::{path::PathBuf, collections::HashMap};

use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use color_eyre::{Result, eyre::eyre};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Database {
    pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new(path: &str) -> Result<Self> {

        if !PathBuf::from(path).exists() {
            tracing::error!("Database doesnt exist, run DATABASE_URL=\"sqlite:./db.sqlite\" sqlx database setup");
            return Err(eyre!("Database doesnt exist"))
        }

        let pool = SqlitePoolOptions::new()
                .max_connections(5)
                .connect(path).await?;
            
        Ok(Self {
            pool: pool
        })
    }

    pub async fn register_user(&mut self, user: &NewUser) -> Result<&mut Self> {
        sqlx::query!(
            "INSERT INTO Users VALUES (?, ?, ?, ?, ?)", 
        user.id, user.username, user.email, user.password, user.session_token)
        .execute(&self.pool).await?;
        
        Ok(self)
    }
}

#[derive(Clone)]
pub struct NewUser {
    pub id: String,
    pub email: String,
    pub username: String,
    pub password: String,
    pub session_token: String
}