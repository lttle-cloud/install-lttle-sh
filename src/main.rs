use axum::response::{IntoResponse, Response};
use axum::{Router, http::StatusCode, routing::get};
use std::env;
use axum::body::Body;

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

    let app = Router::new()
        .route("/", get(|| { get_url(url) }));

    println!("Listening on {}", address);

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_url(url: String) -> impl IntoResponse {
    let outbound_response = reqwest::get(url).await;

    if let Err(err) = outbound_response {
        eprintln!("Error getting url: {:#?}", err);

        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    let outbound_response = outbound_response.unwrap();
    let headers = outbound_response.headers();
    let mut response_builder = Response::builder();

    for (key, value) in headers.iter() {
        response_builder = response_builder.header(key, value);
    }

    let body = outbound_response.bytes().await;

    if let Err(err) = body {
        eprintln!("Error receiving body: {:#?}", err);

        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    match response_builder.body(Body::from(body.unwrap())) {
        Ok(response) => response,
        Err(err) => {
            eprintln!("Error making response: {:#?}", err);

            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
