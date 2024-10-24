mod views;
use axum::{routing::get, Router};
use std::process::exit;
use views::root;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(root));

    let listener = match tokio::net::TcpListener::bind("0.0.0.0:3000").await {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1);
        }
    };

    match axum::serve(listener, app).await {
        Ok(_) => println!("Esto no se va a imprimir"),
        Err(e) => eprintln!("Error: {}", e),
    };
}
