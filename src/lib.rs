use std::sync::Arc;

use sqlx::mysql::MySqlPool;

use config::Config;

pub mod config;
pub mod api;

#[derive(Clone)]
pub struct Context {
    pub config: Arc<Config>,
    pub pool: MySqlPool,
}