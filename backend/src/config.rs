//! Trace configuration. Wraps `ServerConfig::from_env("TRACE")`.
use shared_backend::server::ServerConfig;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct AppConfig(pub Arc<ServerConfig>);

impl AppConfig {
    pub fn load() -> Self {
        Self(Arc::new(ServerConfig::from_env("TRACE")))
    }
}
