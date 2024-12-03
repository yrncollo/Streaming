use routes::router;
use utils::AppState;

pub mod routes;
pub mod utils;
pub async fn launch_server(app_state: AppState){
    let app = router();
    let address = format!("{}:{}", app_state.base_url.url, app_state.base_url.port);

    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
