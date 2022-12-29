use std::sync::Arc;

use sqlx::mysql::MySqlPool;

use config::Config;
use models::cache::Cache;

pub mod config;
pub mod models;
pub mod api;
// pub mod usecases;
pub mod repositories;

#[derive(Clone)]
pub struct Context {
    pub config: Arc<Config>,
    pub cache: Arc<Cache>,
    pub database: MySqlPool,
}