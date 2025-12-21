use crate::services::{NoaaClient, DonkiClient, ExoplanetClient, MLServiceClient};
use crate::database::DatabasePool;
use crate::cache::SpaceWeatherCache;
use crate::api::rate_limit::SharedRateLimiter;
use crate::auth::ApiKeyStore;
use std::sync::Arc;

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub noaa_client: NoaaClient,
    pub donki_client: DonkiClient,
    pub exoplanet_client: ExoplanetClient,
    pub ml_service_client: Option<MLServiceClient>,
    pub db_pool: DatabasePool,
    pub cache: Arc<SpaceWeatherCache>,
    pub rate_limiter: SharedRateLimiter,
    pub api_key_store: ApiKeyStore,
}

impl AppState {
    pub fn new(
        noaa_client: NoaaClient,
        donki_client: DonkiClient,
        exoplanet_client: ExoplanetClient,
        ml_service_client: Option<MLServiceClient>,
        db_pool: DatabasePool,
        cache: SpaceWeatherCache,
        rate_limiter: SharedRateLimiter,
        api_key_store: ApiKeyStore,
    ) -> Self {
        Self { 
            noaa_client,
            donki_client,
            exoplanet_client,
            ml_service_client,
            db_pool, 
            cache: Arc::new(cache),
            rate_limiter,
            api_key_store,
        }
    }
}

