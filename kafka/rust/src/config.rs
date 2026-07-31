#[derive(Debug, Clone)]
pub struct AppConfig {
    pub http_addr: String,
    #[cfg(feature = "kafka")]
    pub kafka_bootstrap_servers: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let http_addr = std::env::var("HTTP_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

        #[cfg(feature = "kafka")]
        let kafka_bootstrap_servers = std::env::var("KAFKA_BOOTSTRAP_SERVERS")
            .unwrap_or_else(|_| "127.0.0.1:9092".to_string());

        Self {
            http_addr,
            #[cfg(feature = "kafka")]
            kafka_bootstrap_servers,
        }
    }
}
