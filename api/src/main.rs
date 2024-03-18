mod playground;

use clap::Parser;
use dataspine::Repo;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    database_url: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let Args { database_url } = Args::parse();

    let repo = Repo::from_database_url(&database_url).await?;
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
