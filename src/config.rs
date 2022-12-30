#[derive(clap::Parser)]
pub struct Config {
    #[clap(short, env)]
    pub server_port: u16,

    #[clap(long, env)]
    pub database_dsn: String,

    #[clap(long, env)]
    pub redis_dsn: String,
}