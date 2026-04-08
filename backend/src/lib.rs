pub mod config;
pub mod errors;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod services;

use dashmap::DashMap;
use std::sync::Arc;
use parking_lot::Mutex;
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct TokenBucket {
    pub tokens: f64,
    pub last_refill: Instant,
}

impl TokenBucket {
    pub fn new(capacity: u32) -> Self {
        TokenBucket {
            tokens: capacity as f64,
            last_refill: Instant::now(),
        }
    }
}

pub type RateLimiterStore = Arc<DashMap<String, Arc<Mutex<TokenBucket>>>>;

pub struct AppState {
    pub db: sqlx::MySqlPool,
    pub jwt_secret: String,
    pub rate_limiter: RateLimiterStore,
}
