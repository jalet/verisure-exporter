use std::net::SocketAddr;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use prometheus_client::encoding::text::encode;
use prometheus_client::registry::Registry;
use tokio::net::TcpListener;
use tracing::{error, info};

pub async fn serve_metrics(
    registry: Arc<Registry>,
    listen_addr: SocketAddr,
    metrics_path: String,
    ready: Arc<AtomicBool>,
) -> anyhow::Result<()> {
    let listener = TcpListener::bind(listen_addr).await?;
    info!("Metrics server listening on http://{}", listen_addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        let registry = registry.clone();
        let metrics_path = metrics_path.clone();
        let ready = ready.clone();

        tokio::spawn(async move {
            let svc = service_fn(move |req: Request<hyper::body::Incoming>| {
                let registry = registry.clone();
                let metrics_path = metrics_path.clone();
                let ready = ready.clone();
                async move {
                    handle_request(req, registry, &metrics_path, ready).await
                }
            });

            if let Err(e) = hyper::server::conn::http1::Builder::new()
                .serve_connection(io, svc)
                .await
            {
                error!("Connection error: {}", e);
            }
        });
    }
}

async fn handle_request(
    req: Request<hyper::body::Incoming>,
    registry: Arc<Registry>,
    metrics_path: &str,
    ready: Arc<AtomicBool>,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    let path = req.uri().path();

    match (req.method(), path) {
        (&Method::GET, p) if p == metrics_path => {
            let mut buf = String::new();
            if let Err(e) = encode(&mut buf, &registry) {
                error!("Failed to encode metrics: {}", e);
                return Ok(Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Full::new(Bytes::from("Failed to encode metrics")))
                    .expect("response builder failed"));
            }
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header(
                    "Content-Type",
                    "application/openmetrics-text; version=1.0.0; charset=utf-8",
                )
                .body(Full::new(Bytes::from(buf)))
                .expect("response builder failed"))
        }
        (&Method::GET, "/healthz") => Ok(Response::builder()
            .status(StatusCode::OK)
            .body(Full::new(Bytes::from("OK")))
            .expect("response builder failed")),
        (&Method::GET, "/readyz") => {
            if ready.load(Ordering::Relaxed) {
                Ok(Response::builder()
                    .status(StatusCode::OK)
                    .body(Full::new(Bytes::from("OK")))
                    .expect("response builder failed"))
            } else {
                Ok(Response::builder()
                    .status(StatusCode::SERVICE_UNAVAILABLE)
                    .body(Full::new(Bytes::from("Not ready")))
                    .expect("response builder failed"))
            }
        }
        (&Method::GET, "/") => {
            let html = format!(
                "<html><head><title>Verisure Exporter</title></head><body>\
                <h1>Verisure Exporter</h1>\
                <p><a href=\"{0}\">{0}</a></p>\
                </body></html>",
                metrics_path
            );
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/html")
                .body(Full::new(Bytes::from(html)))
                .expect("response builder failed"))
        }
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Full::new(Bytes::from("Not Found")))
            .expect("response builder failed")),
    }
}
