// REST API handlers
// This module will contain all API endpoint handlers

pub mod middleware;
pub mod routes;
pub mod handlers;
pub mod rate_limit;
pub mod auth_handlers;
pub mod security;

pub use routes::create_router;
pub use handlers::*;
pub use rate_limit::{SharedRateLimiter, create_rate_limiter};
pub use security::{create_cors_layer, create_request_size_limit_layer, security_headers_middleware, log_security_event};

