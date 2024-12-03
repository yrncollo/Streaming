use axum::{routing::get, Router};

pub fn router() -> Router {
    Router::new()
        .route("/", get(testroute))

    
}
async fn testroute() {
    
    println!("hitting from test route")
}
