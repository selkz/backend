use std::{path::PathBuf, collections::HashMap};

use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use color_eyre::{Result, eyre::eyre};
use tokio_stream::StreamExt;

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

    pub async fn create_user(&mut self, user: &User) -> Result<&mut Self> {
        sqlx::query!(
            "INSERT INTO Users VALUES (?, ?, ?, ?, ?)", 
        user.id, user.username, user.email, user.password, user.session_token)
        .execute(&self.pool).await?;
        Ok(self)
    }
    
    pub async fn get_user(&mut self, email: String) -> Result<User> {
        let mut users = sqlx::query!(
            "SELECT * FROM Users WHERE Email = ?", 
        email)
        .fetch(&self.pool);

        while let Some(row) = users.try_next().await? {
            // map the row into a user-defined domain type
            let email: &str = &row.Email;
            return Ok(User{
                id: row.Id, 
                email: row.Email, 
                username: row.Username, 
                password: row.Password, 
                session_token: row.SessionToken, 
            });
        }
        Err(eyre!("Failed to find user"))
    }
}

#[derive(Clone)]
pub struct User {
    pub id: String,
    pub email: String,
    pub username: String,
    pub password: String,
    pub session_token: String
}