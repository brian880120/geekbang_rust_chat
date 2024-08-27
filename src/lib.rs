mod config;

use axum::Router;
pub use config::AppConfig;

pub fn get_router(config: AppConfig) -> Router {
    todo!()
}