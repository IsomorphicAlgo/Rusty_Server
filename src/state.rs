use crate::services::NoaaClient;
use crate::database::DatabasePool;
use crate::cache::SpaceWeatherCache;
use std::sync::Arc;

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub noaa_client: NoaaClient,
    pub db_pool: DatabasePool,
    pub cache: Arc<SpaceWeatherCache>,
}

impl AppState {
    pub fn new(noaa_client: NoaaClient, db_pool: DatabasePool, cache: SpaceWeatherCache) -> Self {
        Self { 
            noaa_client, 
            db_pool, 
            cache: Arc::new(cache),
        }
    }
}

