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

    let config = Config::parse();
    let redis = redis::Client::open(config.redis_dsn.to_owned()).unwrap();    
    let database = MySqlPool::connect(&config.database_dsn)
        .await
        .unwrap();

    let cache = Arc::new(Cache {
        passwords: Default::default(),
    });

    let config = Arc::new(config);
    let context = Context {
        config,
        cache,
        database,
        redis
    };

    api::serve(context).await?;

    Ok(())
}