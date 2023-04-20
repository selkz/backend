mod db;
mod api;
mod util;

use db::Database;
use api::Api;
use color_eyre::Result;

use tokio;

const DB_URL: &str = "./db.sqlite";
// gen with this (js): "console.log(require('crypto').randomBytes(32).toString('hex'))"
// const SIGN_KEY: &str = "696de9f25b8a582b025156abb9c2a5e37c63fdf3c188ff334d68bdd28ec7be20";

#[tokio::main]
async fn main() -> Result<()>{

    tracing_subscriber::fmt::init();
    tracing::info!("Starting up!");
    tracing::debug!("Debug mode on");
    let database = Database::new(DB_URL).await?;
    
    // database.init_db().await?;

    let mut api = Api::new(database)?;
    api.register_endpoints()?;
    api.listen(([0, 0, 0, 0], 8080)).await?;

    Ok(())
}
