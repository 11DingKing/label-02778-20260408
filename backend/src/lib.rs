pub mod config;
pub mod errors;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod services;

pub struct AppState {
    pub db: sqlx::MySqlPool,
    pub jwt_secret: String,
}
