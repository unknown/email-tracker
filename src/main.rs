use axum::{
    extract::{ConnectInfo, Path, State},
    http::{header, HeaderValue},
    response::IntoResponse,
    routing::get,
    Router,
};
use base64::{engine::general_purpose, Engine as _};
use std::{
    collections::HashMap,
    env,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

type SharedState = Arc<Mutex<AppState>>;

#[derive(Default)]
struct AppState {
    view_counts: HashMap<String, u32>,
}

#[tokio::main]
async fn main() {
    let state = SharedState::default();

    let app = Router::new()
        .route("/tracker/*path", get(tracker))
        .route("/views/*path", get(views))
        .with_state(state);

    let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or("3030".to_string());
    let addr = format!("{host}:{port}");
    println!("Server started, listening on http://{addr}");
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .expect("Failed to start server");
}

async fn tracker(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Path(path): Path<String>,
    State(state): State<SharedState>,
) -> impl IntoResponse {
    let mut lock = state.lock().unwrap();
    let new_view_count = *lock.view_counts.get(&path.clone()).unwrap_or(&0) + 1;
    lock.view_counts.insert(path.clone(), new_view_count);

    println!("Request from {addr} to /{path} ({new_view_count} views)");

    let one_pixel_base64 = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mPk+89QDwADvgGOSHzRgAAAAABJRU5ErkJggg==";
    let one_pixel = general_purpose::STANDARD.decode(one_pixel_base64).unwrap();
    (
        [(header::CONTENT_TYPE, HeaderValue::from_static("image/png"))],
        one_pixel,
    )
}

async fn views(Path(path): Path<String>, State(state): State<SharedState>) -> impl IntoResponse {
    let lock = &state.lock().unwrap();
    let view_count = lock.view_counts.get(&path.clone()).unwrap_or(&0);
    format!("{view_count}")
}
