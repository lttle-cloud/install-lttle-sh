use axum::response::{IntoResponse, Response};
use axum::{Router, http::StatusCode, routing::get};
use std::env;

#[tokio::main]
async fn main() {
    println!("Starting server...");

    let host = env::var("SERVER_HOST").unwrap_or_else(|_| {
        eprintln!("Please set the SERVER_HOST environment variable. Using default 'localhost'");

        "localhost".to_string()
    });

    let port = env::var("SERVER_PORT").unwrap_or_else(|_| {
        eprintln!("Please set the SERVER_PORT environment variable. Using default '8080'.");

        "8080".to_string()
    });

    let url = env::var("INSTALL_URL");

    if url.is_err() {
        panic!("Missing INSTALL_URL environment variable.");
    }

    let url = url.unwrap();

    let address = format!("{}:{}", host, port);

    // build our application with a route
    let app = Router::new()
        .route("/", get(|| { get_url(url) }));

    // run our app with hyper, listening globally on port 3000
    println!("Listening on {}", address);

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_url(url: String) -> impl IntoResponse {
    let response = reqwest::get(url).await;

    if let Err(err) = response {
        eprintln!("Error getting url: {:#?}", err);

        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    let response = response.unwrap();

    let body = response.bytes().await;

    if let Err(err) = body {
        eprintln!("Error receiving body: {:#?}", err);

        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    let body = body.unwrap();

    let response = Response::new(body.into());

    response
}
