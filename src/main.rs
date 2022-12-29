use std::io::Result;
use std::sync::Arc;

use clap::Parser;
use lazer::models::cache::Cache;
use sqlx::mysql::MySqlPool;

use lazer::config::Config;
use lazer::Context;
use lazer::api;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let config = Arc::new(Config::parse());
    
    let database = MySqlPool::connect(&config.database_url)
        .await
        .unwrap();

    let cache = Arc::new(Cache {
        users: Default::default(),
    });
    
    let context = Context {
        config,
        cache,
        database,
    };

    api::serve(context).await?;

    Ok(())
}