mod playground;

use dataspine::Repo;
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;

#[derive(Deserialize)]
struct Config {
    database: DatabaseConfig,
}

#[derive(Deserialize)]
struct DatabaseConfig {
    max_connections: u32,
    url: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config_string = std::fs::read_to_string("config.json")?;
    let config: Config = serde_json::from_str(&config_string)?;

    let pool = PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .connect(&config.database.url)
        .await?;

    let repo = Repo::new(pool);
    let addr = "[::]:50051".parse()?;
    println!("Running playground using http://{addr}");

    tonic::transport::Server::builder()
        .add_service(playground::rpc::games_server::GamesServer::new(
            playground::Server::new(repo),
        ))
        .serve(addr)
        .await?;

    Ok(())
}
