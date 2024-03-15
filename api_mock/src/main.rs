mod games_service;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server_socket_address = "[::]:50051".parse().unwrap();
    println!("Running api mock using http://{server_socket_address}");

    let games_service = games_service::GamesService::new();

    tonic::transport::Server::builder()
        .add_service(games_service::rpc::games_server::GamesServer::new(
            games_service,
        ))
        .serve(server_socket_address)
        .await?;

    Ok(())
}
