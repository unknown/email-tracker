use axum::{
    extract::{ConnectInfo, Path},
    http::{header, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::get,
    Extension, Router,
};
use base64::{engine::general_purpose, Engine as _};
use std::{
    collections::HashMap,
    env,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;

type SharedState = Arc<Mutex<AppState>>;

#[derive(Default)]
struct AppState {
    view_counts: HashMap<String, u32>,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/favicon.ico", get(favicon))
        .route("/*path", get(image))
        .layer(
            ServiceBuilder::new()
                .layer(AddExtensionLayer::new(SharedState::default()))
                .into_inner(),
        );

    let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or("3030".to_string());
    let addr = format!("{host}:{port}");
    println!("Server started, listening on http://{addr}");
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .expect("Failed to start server");
}

async fn favicon() -> impl IntoResponse {
    StatusCode::NOT_FOUND
}

async fn image(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Path(path): Path<String>,
    Extension(state): Extension<SharedState>,
) -> impl IntoResponse {
    let mut lock = state.lock().unwrap();
    let view_count = *lock.view_counts.get(&path.clone()).unwrap_or(&0) + 1;
    lock.view_counts.insert(path.clone(), view_count);

    println!("Request from {addr} to /{path} ({view_count} views)");

    let one_pixel_base64 = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mPk+89QDwADvgGOSHzRgAAAAABJRU5ErkJggg==";
    let one_pixel = general_purpose::STANDARD.decode(one_pixel_base64).unwrap();
    (
        [(header::CONTENT_TYPE, HeaderValue::from_static("image/png"))],
        one_pixel,
    )
}
