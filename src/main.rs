use axum::{
    extract::State,
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use std::sync::Arc;
use std::net::SocketAddr;
use cudarc::driver::{CudaDevice}; 

#[derive(Clone)]
struct AppState {
    device: Arc<CudaDevice>,
}

#[tokio::main]
async fn main() {
    let device = CudaDevice::new(0).unwrap();
    let state = AppState {
        device
    };

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .with_state(state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root(State(state): State<AppState>) -> String {
    state.device.alloc_zeros::<f32>(10).unwrap();
    format!("Hello, World! {:?}", state.device)
}

