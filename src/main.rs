use std::io::Result;
use std::sync::Arc;

use clap::Parser;
use sqlx::mysql::MySqlPool;

use lazer::config::Config;
use lazer::Context;
use lazer::api;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let config = Arc::new(Config::parse());
    let pool = MySqlPool::connect(&config.database_url)
        .await
        .unwrap();

    let context = Context {
        config,
        pool,
    };

    api::serve(context).await?;

    Ok(())
}