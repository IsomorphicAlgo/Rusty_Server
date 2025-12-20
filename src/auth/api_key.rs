// API Key authentication
// Simple API key generation and validation for programmatic access

use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration as ChronoDuration};
use serde::{Serialize, Deserialize};

/// API Key with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub key: String,
    pub name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub last_used: Option<DateTime<Utc>>,
    pub is_active: bool,
}

/// API Key store (in-memory for now, could be database later)
#[derive(Clone)]
pub struct ApiKeyStore {
    keys: Arc<RwLock<HashSet<String>>>,
    key_metadata: Arc<RwLock<std::collections::HashMap<String, ApiKey>>>,
}

impl ApiKeyStore {
    pub fn new() -> Self {
        Self {
            keys: Arc::new(RwLock::new(HashSet::new())),
            key_metadata: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// Generate a new API key
    pub async fn generate_key(&self, name: Option<String>, expires_in_days: Option<u64>) -> ApiKey {
        let key = format!("rs_{}", Uuid::new_v4().to_string().replace("-", ""));
        let now = Utc::now();
        let expires_at = expires_in_days.map(|days| now + ChronoDuration::days(days as i64));

        let api_key = ApiKey {
            key: key.clone(),
            name,
            created_at: now,
            expires_at,
            last_used: None,
            is_active: true,
        };

        // Store the key
        {
            let mut keys = self.keys.write().await;
            keys.insert(key.clone());
        }

        {
            let mut metadata = self.key_metadata.write().await;
            metadata.insert(key.clone(), api_key.clone());
        }

        api_key
    }

    /// Validate an API key
    pub async fn validate_key(&self, key: &str) -> bool {
        let keys = self.keys.read().await;
        if !keys.contains(key) {
            return false;
        }

        // Check metadata for expiration and active status
        let metadata = self.key_metadata.read().await;
        if let Some(api_key) = metadata.get(key) {
            if !api_key.is_active {
                return false;
            }

            if let Some(expires_at) = api_key.expires_at {
                if Utc::now() > expires_at {
                    return false;
                }
            }

            // Update last used (we'll do this in a separate call to avoid deadlock)
            drop(metadata);
            self.update_last_used(key).await;
            return true;
        }

        false
    }

    /// Update last used timestamp for a key
    async fn update_last_used(&self, key: &str) {
        let mut metadata = self.key_metadata.write().await;
        if let Some(api_key) = metadata.get_mut(key) {
            api_key.last_used = Some(Utc::now());
        }
    }

    /// Revoke an API key
    pub async fn revoke_key(&self, key: &str) -> bool {
        let mut keys = self.keys.write().await;
        if keys.remove(key) {
            let mut metadata = self.key_metadata.write().await;
            if let Some(api_key) = metadata.get_mut(key) {
                api_key.is_active = false;
            }
            true
        } else {
            false
        }
    }

    /// List all API keys (for admin purposes)
    pub async fn list_keys(&self) -> Vec<ApiKey> {
        let metadata = self.key_metadata.read().await;
        metadata.values().cloned().collect()
    }
}

impl Default for ApiKeyStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_api_key() {
        let store = ApiKeyStore::new();
        let api_key = store.generate_key(Some("test-key".to_string()), None).await;
        
        assert!(api_key.key.starts_with("rs_"));
        assert_eq!(api_key.name, Some("test-key".to_string()));
        assert!(api_key.is_active);
    }

    #[tokio::test]
    async fn test_validate_api_key() {
        let store = ApiKeyStore::new();
        let api_key = store.generate_key(None, None).await;
        
        assert!(store.validate_key(&api_key.key).await);
        assert!(!store.validate_key("invalid_key").await);
    }

    #[tokio::test]
    async fn test_revoke_api_key() {
        let store = ApiKeyStore::new();
        let api_key = store.generate_key(None, None).await;
        
        assert!(store.validate_key(&api_key.key).await);
        assert!(store.revoke_key(&api_key.key).await);
        assert!(!store.validate_key(&api_key.key).await);
    }
}
