use axum::extract::FromRef;

#[derive(Clone,FromRef)]
pub struct AppState{
    pub base_url: UrlWrapper
}

#[derive(Clone)]
pub struct UrlWrapper{
    pub port: String,
    pub url: String
} 
