// mod game;

// use axum::{
//     http::StatusCode,
//     response::{Html, IntoResponse, Response},
//     routing::{get, post},
//     Router,
// };
// use serde::Deserialize;
// use sqlx::postgres::PgPoolOptions;
// use std::sync::Arc;

// pub mod rpc {
//     tonic::include_proto!("proto.playground"); // The string specified here must match the proto package name
// }

// #[derive(Deserialize)]
// struct Config {
//     database: DatabaseConfig,
// }

// #[derive(Deserialize)]
// struct DatabaseConfig {
//     max_connections: u32,
//     url: String,
// }

// struct AppState {
//     database_pool: Arc<sqlx::PgPool>,
// }

// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     let config_string = std::fs::read_to_string("config.json")?;
//     let config: Config = serde_json::from_str(&config_string)?;

//     let pool = PgPoolOptions::new()
//         .max_connections(config.database.max_connections)
//         .connect(&config.database.url)
//         .await?;

//     let app_state = AppState {
//         database_pool: Arc::new(pool),
//     };

//     let app = Router::new()
//         .route("/", get(game::index))
//         .route("/games/:id/", get(game::show))
//         .route("/games/", post(game::create))
//         .route("/games/:id/scores/", post(game::add_score))
//         .with_state(Arc::new(app_state));

//     let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

//     axum::serve(listener, app).await?;

//     Ok(())
// }

// fn internal_server_error() -> Response {
//     let body = layout(r#"<h1 class="title">500</h1>"#.to_string());

//     (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
// }

// fn not_found() -> Response {
//     let body = layout(r#"<h1 class="title">404</h1>"#.to_string());

//     (StatusCode::NOT_FOUND, body).into_response()
// }

// fn bad_request() -> Response {
//     let body = layout(r#"<h1 class="title">400</h1>"#.to_string());

//     (StatusCode::BAD_REQUEST, body).into_response()
// }

// fn layout(partial: String) -> Html<String> {
//     let html = format!(
//         r#"<!DOCTYPE html>
//     <html>
//     <head>
//         <meta charset="utf-8">
//         <meta name="viewport" content="width=device-width, initial-scale=1">
//         <title>Darts</title>
//         <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bulma@0.9.4/css/bulma.min.css">
//     </head>
//     <body>
//         <section class="section">
//             <div class="container">
//                 <div class="columns is-centered">
//                     <div class="column is-half">
//                         {}
//                     </div>
//                 </div>
//             </div>
//         </section>
//     </body>
//     </html>
//     "#,
//         partial
//     );

//     Html(html)
// }

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
