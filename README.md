# Verisure Exporter for Prometheus

A Prometheus exporter for Verisure alarm systems. Polls the Verisure GraphQL API and exposes metrics for alarm state, climate sensors, door/window sensors, smart locks, smart plugs, and broadband connectivity. Written in Rust.

## Features

- **Verisure GraphQL API integration** with automatic session management and re-auth on 401
- **Auto-detect installation GIID** -- no manual lookup required
- **Alarm state monitoring** -- armed home, armed away, disarmed
- **Climate sensors** -- temperature and humidity from all connected devices
- **Door/window sensors** -- open/closed state with timestamps
- **Smart lock monitoring** -- lock state, motor jam detection, secure mode
- **Smart plug monitoring** -- on/off state
- **Broadband connectivity** -- connected/disconnected state
- **Operational metrics** -- scrape duration, success rate, error counters
- **Health and readiness probes** -- `/healthz` and `/readyz` endpoints
- **JSON structured logging** with configurable log levels
- **Distroless container** -- minimal attack surface, non-root, read-only filesystem
- **Multi-arch builds** -- amd64 and arm64
- **Helm chart** for Kubernetes deployment
- **SBOM and code signing** via cosign

## Getting Started

See the [Getting Started Guide](docs/getting-started.md) for a step-by-step walkthrough.

## Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/` | GET | Landing page |
| `/healthz` | GET | Health check probe |
| `/readyz` | GET | Readiness probe |
| `/metrics` | GET | Prometheus metrics |

## Architecture

```mermaid
graph LR
    A[Verisure Alarm System] --> B[Verisure Cloud]
    B --> C[GraphQL API]
    C --> D[VerisureClient<br/>client.rs]
    D --> E[MetricsCollector<br/>collector.rs]
    E --> F[HTTP Server<br/>server.rs]
    F --> G[/metrics]
    G --> H[Prometheus]
    H --> I[Grafana]
```

## Metrics

### Alarm Metrics

Labels: `installation`

| Metric | Type | Description |
|--------|------|-------------|
| `verisure_alarm_armed_state` | Gauge | Arm state: 0=disarmed, 1=armed_home, 2=armed_away |
| `verisure_alarm_changed_timestamp_seconds` | Gauge | Unix timestamp of last arm state change |

### Climate Metrics

Labels: `installation`, `device_label`, `area`, `device_type`

| Metric | Type | Description |
|--------|------|-------------|
| `verisure_temperature_celsius` | Gauge | Temperature in degrees Celsius |
| `verisure_humidity_percent` | Gauge | Relative humidity percentage |

### Door/Window Metrics

Labels: `installation`, `device_label`, `area`

| Metric | Type | Description |
|--------|------|-------------|
| `verisure_door_window_open` | Gauge | 1=open, 0=closed |
| `verisure_door_window_report_timestamp_seconds` | Gauge | Unix timestamp of last report |

### Lock Metrics

Labels: `installation`, `device_label`, `area`

| Metric | Type | Description |
|--------|------|-------------|
| `verisure_lock_locked` | Gauge | 1=locked, 0=unlocked |
| `verisure_lock_motor_jam` | Gauge | 1 if motor jammed |
| `verisure_lock_secure_mode` | Gauge | 1 if secure mode active |

### Smart Plug Metrics

Labels: `installation`, `device_label`, `area`

| Metric | Type | Description |
|--------|------|-------------|
| `verisure_smartplug_on` | Gauge | 1=on, 0=off |

### Broadband Metrics

Labels: `installation`

| Metric | Type | Description |
|--------|------|-------------|
| `verisure_broadband_connected` | Gauge | 1 if broadband connected |

### Operational Metrics

| Metric | Type | Description |
|--------|------|-------------|
| `verisure_scrape_duration_seconds` | Gauge | Duration of the last scrape in seconds |
| `verisure_scrape_success` | Gauge | 1 if last scrape succeeded |
| `verisure_scrape_errors_total` | Counter | Total number of scrape errors |

## Configuration

All settings can be provided as environment variables or CLI flags.

| Variable | Default | Purpose |
|----------|---------|---------|
| `VERISURE_USERNAME` | -- | Verisure account email |
| `VERISURE_PASSWORD` | -- | Verisure account password |
| `VERISURE_GIID` | auto-detect | Installation GIID |
| `VERISURE_API_URL` | `https://automation01.verisure.com` | API base URL |
| `LISTEN_ADDRESS` | `0.0.0.0:9878` | HTTP listen address |
| `METRICS_PATH` | `/metrics` | Metrics endpoint path |
| `POLL_INTERVAL` | `60` | Seconds between polls |
| `LOG_LEVEL` | `info` | Log level (trace, debug, info, warn, error) |

## Security

- All communication with the Verisure API uses TLS
- Authentication via basic auth with session cookies (automatic re-auth on 401)
- Distroless container image with no shell or package manager
- Runs as non-root user
- Read-only root filesystem
- Container images signed with cosign

## Building from Source

```bash
# Build
cargo build --release

# Run
export VERISURE_USERNAME=your@email.com
export VERISURE_PASSWORD=yourpassword
cargo run --release

# Run tests
cargo test
```

## Docker

```bash
# Build the image
docker build -t verisure-exporter:dev .

# Run
docker run -d \
  -e VERISURE_USERNAME=your@email.com \
  -e VERISURE_PASSWORD=yourpassword \
  -p 9878:9878 \
  verisure-exporter:dev
```

## Development

### Project Structure

```
verisure-exporter/
  src/
    main.rs              # Entry point, CLI parsing, startup
    config.rs            # Configuration struct and loading
    server.rs            # HTTP server, routing, health probes
    metrics/
      mod.rs             # Metrics module exports
      collector.rs       # Prometheus metrics collection
      registry.rs        # Prometheus registry setup
    verisure/
      mod.rs             # Verisure module exports
      client.rs          # GraphQL API client, auth, session management
      queries.rs         # GraphQL query definitions
      types.rs           # API response types and deserialization
  charts/
    verisure-exporter/   # Helm chart
  deploy/                # Raw Kubernetes manifests (ExternalSecrets + OpenBao)
  Dockerfile             # Multi-stage distroless build
```

### Local Development

```bash
export VERISURE_USERNAME=your@email.com
export VERISURE_PASSWORD=yourpassword
export LOG_LEVEL=debug
cargo run
```

The exporter will start on `http://localhost:9878`. Visit `/metrics` to see Prometheus metrics.

## Deployment

### Kubernetes with Helm

See the [Helm Deployment Guide](docs/helm-deployment.md) for full instructions.

```bash
helm install verisure-exporter oci://ghcr.io/jalet/verisure-exporter/verisure-exporter
```

### Docker Compose

```yaml
services:
  verisure-exporter:
    image: ghcr.io/jalet/verisure-exporter:latest
    ports:
      - "9878:9878"
    environment:
      - VERISURE_USERNAME=your@email.com
      - VERISURE_PASSWORD=yourpassword
    restart: unless-stopped
    read_only: true
    security_opt:
      - no-new-privileges:true
```

### Raw Kubernetes Manifests

The `deploy/` directory contains raw Kubernetes manifests including an `ExternalSecret` resource for organizations using OpenBao as a secrets backend.

```bash
kubectl apply -f deploy/
```

## Monitoring

### Prometheus Configuration

```yaml
scrape_configs:
  - job_name: verisure
    scrape_interval: 60s
    static_configs:
      - targets:
          - verisure-exporter:9878
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| **MFA / two-factor authentication** | MFA is **not supported**. You must disable MFA on the Verisure account used by the exporter. Consider creating a dedicated account without MFA. |
| **Authentication failures** | Verify `VERISURE_USERNAME` and `VERISURE_PASSWORD` are correct. Check that you can log in to the Verisure app with the same credentials. |
| **Rate limiting** | The Verisure API may rate-limit frequent requests. Increase `POLL_INTERVAL` if you see repeated errors. The default of 60 seconds is generally safe. |
| **No metrics appearing** | Check `/healthz` and `/readyz` first. Look at the exporter logs for errors. Ensure the Verisure account has at least one installation. |
| **GIID auto-detection fails** | Set `VERISURE_GIID` explicitly. You can find your GIID in the Verisure app or via the API. |

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Commit your changes
4. Push to the branch (`git push origin feature/my-feature`)
5. Open a Pull Request

## License

This project is licensed under the [MIT License](LICENSE).

## Support

- Open an [issue](https://github.com/jalet/verisure-exporter/issues) for bugs or feature requests
- Check the [troubleshooting](#troubleshooting) section for common issues
- Review the [getting started guide](docs/getting-started.md) for setup help
