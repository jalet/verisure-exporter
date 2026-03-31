use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(name = "verisure-exporter", about = "Prometheus exporter for Verisure alarm systems")]
pub struct Config {
    #[arg(long, env = "VERISURE_USERNAME")]
    pub username: String,

    #[arg(long, env = "VERISURE_PASSWORD")]
    pub password: String,

    #[arg(long, env = "VERISURE_GIID")]
    pub giid: Option<String>,

    #[arg(long, env = "LISTEN_ADDRESS", default_value = "0.0.0.0:9878")]
    pub listen_address: String,

    #[arg(long, env = "METRICS_PATH", default_value = "/metrics")]
    pub metrics_path: String,

    #[arg(long, env = "POLL_INTERVAL", default_value = "60")]
    pub poll_interval: u64,

    #[arg(long, env = "VERISURE_API_URL", default_value = "https://m-api01.verisure.com")]
    pub api_url: String,

    #[arg(long, env = "LOG_LEVEL", default_value = "info")]
    pub log_level: String,
}
