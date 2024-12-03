use dotenvy_macro::dotenv;
use streaming::{launch_server, utils::{AppState, UrlWrapper}};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let port = dotenv!("PORT").to_string();
    let base_url = dotenv!("URL").to_string();
    let app_state = AppState{
        base_url: UrlWrapper{
            port,
            url: base_url
        }
    };
    launch_server(app_state).await;
}
