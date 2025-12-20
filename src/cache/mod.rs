// Caching layer for space weather data
// Uses moka for high-performance in-memory caching with TTL support

use moka::future::Cache;
use std::sync::Arc;
use std::time::Duration;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use serde::{Serialize, Deserialize};
use crate::models::SpaceWeatherResponse;
use crate::config::CacheConfig;

/// Cache key for current conditions (singleton key)
const CURRENT_CONDITIONS_KEY: &str = "current_conditions";

/// Cache key for alerts (singleton key)
const ALERTS_KEY: &str = "alerts";

/// Historical query cache key (derived from query parameters)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct HistoricalQueryKey {
    start_date: Option<String>,
    end_date: Option<String>,
    data_type: Option<String>,
    limit: Option<u32>,
}

impl HistoricalQueryKey {
    fn from_query_params(
        start_date: Option<&String>,
        end_date: Option<&String>,
        data_type: Option<&String>,
        limit: Option<u32>,
    ) -> Self {
        Self {
            start_date: start_date.cloned(),
            end_date: end_date.cloned(),
            data_type: data_type.cloned(),
            limit,
        }
    }
}

/// Cache metrics for monitoring
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CacheMetrics {
    pub current_conditions_hits: u64,
    pub current_conditions_misses: u64,
    pub historical_hits: u64,
    pub historical_misses: u64,
    pub alerts_hits: u64,
    pub alerts_misses: u64,
}

/// Main cache structure for space weather data
pub struct SpaceWeatherCache {
    /// Cache for current conditions (short TTL)
    current_conditions: Cache<String, SpaceWeatherResponse>,
    
    /// Cache for historical data queries (longer TTL)
    historical: Cache<String, Vec<SpaceWeatherResponse>>,
    
    /// Cache for alerts (short TTL)
    alerts: Cache<String, Vec<SpaceWeatherResponse>>,
    
    /// Cache metrics
    metrics: Arc<tokio::sync::RwLock<CacheMetrics>>,
}

impl SpaceWeatherCache {
    /// Create a new cache with the given configuration
    pub fn new(config: &CacheConfig) -> Self {
        // Calculate max capacity based on max_size_mb
        // Estimate: each SpaceWeatherResponse is roughly 1-2KB, so 100MB ≈ 50,000-100,000 entries
        // We'll use a conservative estimate and split capacity across caches
        let max_capacity = (config.max_size_mb * 1024 * 1024 / 2048) as u64; // ~2KB per entry
        let current_capacity = max_capacity / 4; // 25% for current conditions
        let historical_capacity = max_capacity / 2; // 50% for historical
        let alerts_capacity = max_capacity / 4; // 25% for alerts

        let current_conditions = Cache::builder()
            .max_capacity(current_capacity)
            .time_to_live(Duration::from_secs(config.current_conditions_ttl_seconds))
            .build();

        let historical = Cache::builder()
            .max_capacity(historical_capacity)
            .time_to_live(Duration::from_secs(config.historical_data_ttl_seconds))
            .build();

        let alerts = Cache::builder()
            .max_capacity(alerts_capacity)
            .time_to_live(Duration::from_secs(config.alerts_ttl_seconds))
            .build();

        Self {
            current_conditions,
            historical,
            alerts,
            metrics: Arc::new(tokio::sync::RwLock::new(CacheMetrics::default())),
        }
    }

    /// Get current conditions from cache
    pub async fn get_current_conditions(&self) -> Option<SpaceWeatherResponse> {
        match self.current_conditions.get(&CURRENT_CONDITIONS_KEY.to_string()).await {
            Some(response) => {
                self.increment_hit("current_conditions").await;
                Some(response)
            }
            None => {
                self.increment_miss("current_conditions").await;
                None
            }
        }
    }

    /// Store current conditions in cache
    pub async fn set_current_conditions(&self, response: SpaceWeatherResponse) {
        self.current_conditions
            .insert(CURRENT_CONDITIONS_KEY.to_string(), response)
            .await;
    }

    /// Get historical data from cache
    pub async fn get_historical(
        &self,
        start_date: Option<&String>,
        end_date: Option<&String>,
        data_type: Option<&String>,
        limit: Option<u32>,
    ) -> Option<Vec<SpaceWeatherResponse>> {
        let key = Self::historical_key(start_date, end_date, data_type, limit);
        match self.historical.get(&key).await {
            Some(responses) => {
                self.increment_hit("historical").await;
                Some(responses)
            }
            None => {
                self.increment_miss("historical").await;
                None
            }
        }
    }

    /// Store historical data in cache
    pub async fn set_historical(
        &self,
        start_date: Option<&String>,
        end_date: Option<&String>,
        data_type: Option<&String>,
        limit: Option<u32>,
        responses: Vec<SpaceWeatherResponse>,
    ) {
        let key = Self::historical_key(start_date, end_date, data_type, limit);
        self.historical.insert(key, responses).await;
    }

    /// Get alerts from cache
    pub async fn get_alerts(&self) -> Option<Vec<SpaceWeatherResponse>> {
        match self.alerts.get(&ALERTS_KEY.to_string()).await {
            Some(responses) => {
                self.increment_hit("alerts").await;
                Some(responses)
            }
            None => {
                self.increment_miss("alerts").await;
                None
            }
        }
    }

    /// Store alerts in cache
    pub async fn set_alerts(&self, responses: Vec<SpaceWeatherResponse>) {
        self.alerts.insert(ALERTS_KEY.to_string(), responses).await;
    }

    /// Invalidate current conditions cache
    pub async fn invalidate_current_conditions(&self) {
        self.current_conditions.invalidate(&CURRENT_CONDITIONS_KEY.to_string()).await;
    }

    /// Invalidate alerts cache
    pub async fn invalidate_alerts(&self) {
        self.alerts.invalidate(&ALERTS_KEY.to_string()).await;
    }

    /// Invalidate all caches
    pub async fn invalidate_all(&self) {
        self.current_conditions.invalidate_all();
        self.historical.invalidate_all();
        self.alerts.invalidate_all();
    }

    /// Get cache metrics
    pub async fn get_metrics(&self) -> CacheMetrics {
        self.metrics.read().await.clone()
    }

    /// Get cache statistics
    pub fn get_stats(&self) -> CacheStats {
        CacheStats {
            current_conditions_size: self.current_conditions.weighted_size(),
            historical_size: self.historical.weighted_size(),
            alerts_size: self.alerts.weighted_size(),
        }
    }

    /// Generate a cache key for historical queries
    fn historical_key(
        start_date: Option<&String>,
        end_date: Option<&String>,
        data_type: Option<&String>,
        limit: Option<u32>,
    ) -> String {
        let query_key = HistoricalQueryKey::from_query_params(start_date, end_date, data_type, limit);
        // Use hash to create a shorter key
        let mut hasher = DefaultHasher::new();
        query_key.hash(&mut hasher);
        format!("historical:{}", hasher.finish())
    }

    /// Increment hit counter for a cache type
    async fn increment_hit(&self, cache_type: &str) {
        let mut metrics = self.metrics.write().await;
        match cache_type {
            "current_conditions" => metrics.current_conditions_hits += 1,
            "historical" => metrics.historical_hits += 1,
            "alerts" => metrics.alerts_hits += 1,
            _ => {}
        }
    }

    /// Increment miss counter for a cache type
    async fn increment_miss(&self, cache_type: &str) {
        let mut metrics = self.metrics.write().await;
        match cache_type {
            "current_conditions" => metrics.current_conditions_misses += 1,
            "historical" => metrics.historical_misses += 1,
            "alerts" => metrics.alerts_misses += 1,
            _ => {}
        }
    }
}

/// Cache statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    pub current_conditions_size: u64,
    pub historical_size: u64,
    pub alerts_size: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::*;
    use chrono::Utc;

    fn create_test_response() -> SpaceWeatherResponse {
        SpaceWeatherResponse {
            data: SpaceWeatherData {
                solar_flare: None,
                geomagnetic_storm: None,
                radiation: None,
                solar_wind: None,
                kp_index: Some(KpIndex {
                    value: 3.0,
                    level: "Quiet".to_string(),
                    timestamp: Utc::now(),
                }),
            },
            metadata: ResponseMetadata {
                timestamp: Utc::now(),
                source: "test".to_string(),
                cached: false,
            },
        }
    }

    #[tokio::test]
    async fn test_current_conditions_cache() {
        let config = CacheConfig::default();
        let cache = SpaceWeatherCache::new(&config);

        // Cache miss initially
        assert!(cache.get_current_conditions().await.is_none());

        // Store and retrieve
        let response = create_test_response();
        cache.set_current_conditions(response.clone()).await;
        
        let cached = cache.get_current_conditions().await;
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().metadata.source, "test");

        // Check metrics
        let metrics = cache.get_metrics().await;
        assert_eq!(metrics.current_conditions_hits, 1);
        assert_eq!(metrics.current_conditions_misses, 1);
    }

    #[tokio::test]
    async fn test_historical_cache() {
        let config = CacheConfig::default();
        let cache = SpaceWeatherCache::new(&config);

        let start = Some("2024-01-01T00:00:00Z".to_string());
        let end = Some("2024-01-02T00:00:00Z".to_string());

        // Cache miss initially
        assert!(cache.get_historical(start.as_ref(), end.as_ref(), None, None).await.is_none());

        // Store and retrieve
        let responses = vec![create_test_response()];
        cache.set_historical(start.as_ref(), end.as_ref(), None, None, responses.clone()).await;
        
        let cached = cache.get_historical(start.as_ref(), end.as_ref(), None, None).await;
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().len(), 1);

        // Different query should miss
        let different_end = Some("2024-01-03T00:00:00Z".to_string());
        assert!(cache.get_historical(start.as_ref(), different_end.as_ref(), None, None).await.is_none());
    }

    #[tokio::test]
    async fn test_alerts_cache() {
        let config = CacheConfig::default();
        let cache = SpaceWeatherCache::new(&config);

        // Cache miss initially
        assert!(cache.get_alerts().await.is_none());

        // Store and retrieve
        let responses = vec![create_test_response()];
        cache.set_alerts(responses.clone()).await;
        
        let cached = cache.get_alerts().await;
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_cache_invalidation() {
        let config = CacheConfig::default();
        let cache = SpaceWeatherCache::new(&config);

        // Store data
        let response = create_test_response();
        cache.set_current_conditions(response.clone()).await;
        cache.set_alerts(vec![response.clone()]).await;

        // Verify it's cached
        assert!(cache.get_current_conditions().await.is_some());
        assert!(cache.get_alerts().await.is_some());

        // Invalidate
        cache.invalidate_current_conditions().await;
        cache.invalidate_alerts().await;

        // Verify it's gone
        assert!(cache.get_current_conditions().await.is_none());
        assert!(cache.get_alerts().await.is_none());
    }

    #[tokio::test]
    async fn test_cache_metrics() {
        let config = CacheConfig::default();
        let cache = SpaceWeatherCache::new(&config);

        // Generate some cache activity
        let response = create_test_response();
        
        // Initial miss (cache is empty)
        cache.get_current_conditions().await; // miss
        
        // Set and retrieve
        cache.set_current_conditions(response.clone()).await;
        cache.get_current_conditions().await; // hit
        cache.get_current_conditions().await; // hit
        cache.invalidate_current_conditions().await;
        cache.get_current_conditions().await; // miss

        let metrics = cache.get_metrics().await;
        assert_eq!(metrics.current_conditions_hits, 2);
        assert_eq!(metrics.current_conditions_misses, 2);
    }
}
