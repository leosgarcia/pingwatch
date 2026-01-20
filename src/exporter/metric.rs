use prometheus::{CounterVec, HistogramVec, HistogramOpts, Opts, Registry, TextEncoder};
use std::sync::Arc;

/// Prometheus metrics collector
#[derive(Debug, Clone)]
pub struct PrometheusMetrics {
    /// Ping latency histogram metric
    ping_duration_histogram: HistogramVec,
    /// Total number of ping requests (grouped by status)
    ping_requests_total: CounterVec,
    /// Prometheus registry
    registry: Arc<Registry>,
}

impl PrometheusMetrics {
    /// Creates a new Prometheus metrics collector
    pub fn new() -> Result<Self, prometheus::Error> {
        // Create registry
        let registry = Arc::new(Registry::new());

        // Define latency buckets (in seconds): 1ms, 5ms, 10ms, 50ms, 100ms, 500ms, 1s, 5s, 10s, +Inf
        let buckets = vec![
            0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0, 5.0, 10.0,
        ];

        // Create histogram metric
        let ping_duration_histogram = HistogramVec::new(
            HistogramOpts::new(
                "nbping_ping_duration_seconds",
                "Histogram of ping durations in seconds",
            )
                .buckets(buckets),
            &["target", "ip"], // label names
        )?;

        // Create counter for total ping requests
        let ping_requests_total = CounterVec::new(
            Opts::new(
                "nbping_ping_requests_total",
                "Total number of ping requests",
            ),
            &["target", "ip", "status"],
        )?;

        // Register metrics
        registry.register(Box::new(ping_duration_histogram.clone()))?;
        registry.register(Box::new(ping_requests_total.clone()))?;

        Ok(Self {
            ping_duration_histogram,
            ping_requests_total,
            registry,
        })
    }

    /// Records a successful ping (records to histogram)
    pub fn record_ping_success(&self, target: &str, ip: &str, rtt_ms: f64) {
        let rtt_seconds = rtt_ms / 1000.0;

        self.ping_requests_total
            .with_label_values(&[target, ip, "success"])
            .inc();

        // Add labels to histogram and observe value
        self.ping_duration_histogram
            .with_label_values(&[target, ip])
            .observe(rtt_seconds);
    }

    /// Records a timed-out ping (not recorded in histogram, but other metrics can be added here)
    pub fn record_ping_timeout(&self, target: &str, ip: &str) {
        self.ping_requests_total
            .with_label_values(&[target, ip, "timeout"])
            .inc();
    }

    /// Records a failed ping
    pub fn record_ping_error(&self, target: &str, ip: &str) {
        self.ping_requests_total
            .with_label_values(&[target, ip, "error"])
            .inc();
    }

    /// Gets metrics data in Prometheus format
    pub fn gather(&self) -> String {
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();

        encoder.encode_to_string(&metric_families).unwrap_or_else(|e| {
            eprintln!("Error encoding metrics: {}", e);
            String::new()
        })
    }

}

impl Default for PrometheusMetrics {
    fn default() -> Self {
        Self::new().expect("Failed to create PrometheusMetrics")
    }
}

/// HTTP server to expose /metrics endpoint
pub mod http_server {
    use super::*;
    use hyper::service::service_fn;
    use hyper::{Method, Request, Response, StatusCode};
    use hyper_util::rt::TokioIo;
    use hyper_util::server::conn::auto::Builder;
    use http_body_util::Full;
    use hyper::body::Bytes;
    use std::convert::Infallible;
    use std::net::SocketAddr;
    use std::sync::Arc;
    use tokio::net::TcpListener;

    /// Starts Prometheus metrics HTTP server with graceful shutdown support
    pub async fn start_metrics_server(
        metrics: Arc<PrometheusMetrics>,
        addr: SocketAddr,
        mut shutdown_rx: tokio::sync::oneshot::Receiver<()>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let listener = TcpListener::bind(addr).await?;

        loop {
            tokio::select! {
                // Accept new connections
                accept_result = listener.accept() => {
                    match accept_result {
                        Ok((stream, _)) => {
                            let metrics = metrics.clone();
                            
                            tokio::task::spawn(async move {
                                let io = TokioIo::new(stream);
                                let service = service_fn(move |req| {
                                    handle_request(req, metrics.clone())
                                });

                                if let Err(err) = Builder::new(hyper_util::rt::TokioExecutor::new())
                                    .serve_connection(io, service)
                                    .await
                                {
                                    eprintln!("Error serving connection: {:?}", err);
                                }
                            });
                        }
                        Err(e) => {
                            eprintln!("Failed to accept connection: {}", e);
                        }
                    }
                }
                // Receive shutdown signal
                _ = &mut shutdown_rx => {
                    println!("Metrics server shutting down gracefully");
                    break;
                }
            }
        }

        Ok(())
    }

    /// Handles HTTP requests
    async fn handle_request(
        req: Request<hyper::body::Incoming>,
        metrics: Arc<PrometheusMetrics>,
    ) -> Result<Response<Full<Bytes>>, Infallible> {
        match (req.method(), req.uri().path()) {
            (&Method::GET, "/metrics") => {
                let metrics_output = metrics.gather();
                Ok(Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "text/plain; charset=utf-8")
                    .body(Full::new(Bytes::from(metrics_output)))
                    .unwrap())
            }
            (&Method::GET, "/") => {
                let body = r#"<html>
<head><title>PingWatch Metrics</title></head>
<body>
<h1>PingWatch Metrics</h1>
<p><a href='/metrics'>Metrics</a></p>
</body>
</html>"#;
                Ok(Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "text/html")
                    .body(Full::new(Bytes::from(body)))
                    .unwrap())
            }
            _ => {
                Ok(Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Full::new(Bytes::from("Not Found")))
                    .unwrap())
            }
        }
    }
}
