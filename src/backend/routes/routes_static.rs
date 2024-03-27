use axum::{routing::get_service, Router};
use tower_http::services::ServeDir;
/// Serve static files from file directory
pub fn routes() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
