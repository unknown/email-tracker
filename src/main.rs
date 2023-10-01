use axum::{
    extract::ConnectInfo,
    http::{header, HeaderValue},
    response::IntoResponse,
    routing::get,
    Router,
};
use base64::{engine::general_purpose, Engine as _};
use std::{env, net::SocketAddr};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/image", get(image));

    let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or("3030".to_string());
    let addr = format!("{host}:{port}");
    println!("Server started, listening on http://{addr}");
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .expect("Failed to start server");
}

async fn image(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> impl IntoResponse {
    println!("Request from {addr}");
    let one_pixel_base64 = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mPk+89QDwADvgGOSHzRgAAAAABJRU5ErkJggg==";
    let one_pixel = general_purpose::STANDARD.decode(one_pixel_base64).unwrap();
    (
        [(header::CONTENT_TYPE, HeaderValue::from_static("image/png"))],
        one_pixel,
    )
}
