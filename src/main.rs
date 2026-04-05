mod config;
mod metrics;
mod server;
mod verisure;

use std::net::SocketAddr;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::{Duration, Instant};

use clap::Parser;
use prometheus_client::registry::Registry;
use tracing::{error, info};

use config::Config;
use metrics::{collector, Metrics};
use verisure::VerisureClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::parse();

    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(&config.log_level));
    tracing_subscriber::fmt().with_env_filter(filter).init();

    info!("Starting verisure-exporter");

    let mut registry = Registry::default();
    let metrics = Arc::new(Metrics::new(&mut registry));
    let registry = Arc::new(registry);

    let client = Arc::new(VerisureClient::new(&config).await?);
    client.init().await?;

    if config.api_introspection {
        for type_name in &["Climate", "SmartLock", "SmartPlug", "Installation"] {
            println!("=== {} ===", type_name);
            println!("{}", client.introspect(type_name).await?);
        }
        return Ok(());
    }

    let giid = client
        .get_giid()
        .await
        .expect("GIID must be set after init");
    info!(giid = %giid, "Using installation");

    let ready = Arc::new(AtomicBool::new(false));

    {
        let client = client.clone();
        let metrics = metrics.clone();
        let ready = ready.clone();
        let giid = giid.clone();
        let interval = Duration::from_secs(config.poll_interval);

        tokio::spawn(async move {
            scrape_loop(client, metrics, giid, interval, ready).await;
        });
    }

    let listen_addr: SocketAddr = config.listen_address.parse()?;
    server::serve_metrics(registry, listen_addr, config.metrics_path, ready).await?;

    Ok(())
}

async fn scrape_loop(
    client: Arc<VerisureClient>,
    metrics: Arc<Metrics>,
    giid: String,
    interval: Duration,
    ready: Arc<AtomicBool>,
) {
    let mut ticker = tokio::time::interval(interval);
    ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

    loop {
        ticker.tick().await;
        let start = Instant::now();

        match client.fetch_all().await {
            Ok(data) => {
                collector::update_metrics(&data, &metrics, &giid);
                metrics.scrape_success.set(1);
                ready.store(true, Ordering::Relaxed);
                info!("Scrape completed successfully");
            }
            Err(e) => {
                metrics.scrape_success.set(0);
                metrics.scrape_errors_total.inc();
                error!(error = %e, "Scrape failed");
            }
        }

        metrics
            .scrape_duration_seconds
            .set(start.elapsed().as_secs_f64());
    }
}
