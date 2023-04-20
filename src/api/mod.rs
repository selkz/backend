pub mod routes;

use color_eyre::Result;

use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

use crate::db::Database;

#[derive(Debug, Clone)]
pub struct Api{
    router: Router<>,
    db: Database,
}

impl Api {
    pub fn new(db: Database) -> Result<Self> {
        
        let router = Router::new();
        Ok(Self{
            router,
            db
        })
    }

    pub fn register_endpoints(&mut self) -> Result<&mut Self> {
        let self_auth_post = self.clone();
        let self_auth_get = self.clone();
        self.router = self.router.clone()
            .route("/api", get(routes::root))
            .route("/api/auth", post(move |body: Json<routes::auth::post::Input>| {
                routes::auth::post::handler(body, self_auth_post.clone())
            }))
            .route("/api/auth", get(move |body: Json<routes::auth::get::Input>| {
                routes::auth::get::handler(body, self_auth_get.clone())
            }))
            ;

        Ok(self)
    }

    pub async fn listen(&mut self, addr: ([u8; 4], u16)) -> Result<&mut Api<>> {
        


        let sock_addr = SocketAddr::from(addr);
        tracing::info!("listening on http://{}", format!("{}.{}.{}.{}:{}", addr.0[0], addr.0[1], addr.0[2],addr.0[3], addr.1));

        axum::Server::bind(&sock_addr)
            .serve(self.router.clone().into_make_service())
            .await
            .unwrap();

        Ok(self)
    }

}

