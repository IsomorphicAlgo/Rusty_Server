use crate::services::NoaaClient;
use crate::database::DatabasePool;
use crate::cache::SpaceWeatherCache;
use crate::api::rate_limit::SharedRateLimiter;
use crate::auth::ApiKeyStore;
use std::sync::Arc;

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub noaa_client: NoaaClient,
    pub db_pool: DatabasePool,
    pub cache: Arc<SpaceWeatherCache>,
    pub rate_limiter: SharedRateLimiter,
    pub api_key_store: ApiKeyStore,
}

impl AppState {
    pub fn new(
        noaa_client: NoaaClient,
        db_pool: DatabasePool,
        cache: SpaceWeatherCache,
        rate_limiter: SharedRateLimiter,
        api_key_store: ApiKeyStore,
    ) -> Self {
        Self { 
            noaa_client, 
            db_pool, 
            cache: Arc::new(cache),
            rate_limiter,
            api_key_store,
        }
    }
}

