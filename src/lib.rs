use routes::router;

pub mod routes;
pub async fn launch_server(){
    let app = router();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
