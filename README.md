# verisure-exporter

Prometheus exporter for Verisure alarm systems. Polls the Verisure GraphQL API and exposes metrics for alarm state, climate sensors, door/window sensors, smart locks, smart plugs, and broadband connectivity.

## Metrics

| Metric | Type | Description |
|--------|------|-------------|
| `verisure_alarm_armed_state` | Gauge | Arm state: 0=disarmed, 1=armed_home, 2=armed_away |
| `verisure_alarm_changed_timestamp_seconds` | Gauge | Unix timestamp of last arm state change |
| `verisure_temperature_celsius` | Gauge | Temperature in °C |
| `verisure_humidity_percent` | Gauge | Relative humidity % |
| `verisure_door_window_open` | Gauge | 1=open, 0=closed |
| `verisure_lock_locked` | Gauge | 1=locked, 0=unlocked |
| `verisure_lock_motor_jam` | Gauge | 1 if motor jammed |
| `verisure_lock_secure_mode` | Gauge | 1 if secure mode active |
| `verisure_smartplug_on` | Gauge | 1=on, 0=off |
| `verisure_broadband_connected` | Gauge | 1 if broadband connected |
| `verisure_scrape_duration_seconds` | Gauge | Last scrape duration |
| `verisure_scrape_success` | Gauge | 1 if last scrape succeeded |
| `verisure_scrape_errors_total` | Counter | Total scrape errors |

## Configuration

| Environment Variable | Flag | Default | Description |
|---------------------|------|---------|-------------|
| `VERISURE_USERNAME` | `--username` | required | Verisure account email |
| `VERISURE_PASSWORD` | `--password` | required | Verisure account password |
| `VERISURE_GIID` | `--giid` | auto-detect | Installation GIID |
| `LISTEN_ADDRESS` | `--listen-address` | `0.0.0.0:9878` | HTTP listen address |
| `METRICS_PATH` | `--metrics-path` | `/metrics` | Metrics endpoint path |
| `POLL_INTERVAL` | `--poll-interval` | `60` | Seconds between polls |
| `VERISURE_API_URL` | `--api-url` | `https://m-api01.verisure.com` | API base URL |
| `LOG_LEVEL` | `--log-level` | `info` | Log level |

## Kubernetes Deployment

Prerequisites: External Secrets Operator with an OpenBao ClusterSecretStore named `openbao`.

Store credentials in OpenBao at `secret/data/verisure` with keys `username`, `password`, `giid`.

```bash
kubectl apply -f deploy/
```

## Local Development

```bash
export VERISURE_USERNAME=your@email.com
export VERISURE_PASSWORD=yourpassword
cargo run
```
