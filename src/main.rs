use streaming::launch_server;

#[tokio::main]
async fn main() {
    launch_server().await;
}
