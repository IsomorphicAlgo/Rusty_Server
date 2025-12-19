use crate::services::NoaaClient;

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub noaa_client: NoaaClient,
}

impl AppState {
    pub fn new(noaa_client: NoaaClient) -> Self {
        Self { noaa_client }
    }
}

