# Getting Started with Verisure Exporter

This guide walks you through setting up the Verisure Exporter for Prometheus from scratch.

## Prerequisites

- A **Verisure account** (the email and password you use to log in to the Verisure app)
- A **Verisure alarm system** registered to your account
- **Docker** (recommended) or **Rust 1.94+** for building from source
- **curl** for testing

> **WARNING**: Multi-factor authentication (MFA) must be **disabled** on the Verisure account used by the exporter. The exporter authenticates via basic auth against the Verisure GraphQL API and cannot handle MFA challenges. Consider creating a dedicated Verisure account without MFA for the exporter.

## Step 1: Find Your Verisure Credentials

You need the same credentials you use to log in to the Verisure app:

- **Username**: Your Verisure account email address
- **Password**: Your Verisure account password

These will be passed to the exporter as `VERISURE_USERNAME` and `VERISURE_PASSWORD`.

## Step 2: Find Your Installation GIID (Optional)

The exporter **auto-detects** your installation GIID on startup. If you have multiple installations or want to specify one explicitly, you can set `VERISURE_GIID`.

To find your GIID, check the exporter logs on first startup -- it will log the detected GIID:

```
{"level":"info","message":"Auto-detected installation GIID: 123456789012"}
```

For most users, auto-detection works and this step can be skipped entirely.

## Step 3: Run the Exporter with Docker

```bash
docker run -d \
  --name verisure-exporter \
  -e VERISURE_USERNAME=your@email.com \
  -e VERISURE_PASSWORD=yourpassword \
  -p 9878:9878 \
  --read-only \
  --security-opt no-new-privileges:true \
  ghcr.io/jalet/verisure-exporter:latest
```

Or, to build and run from source:

```bash
git clone https://github.com/jalet/verisure-exporter.git
cd verisure-exporter
export VERISURE_USERNAME=your@email.com
export VERISURE_PASSWORD=yourpassword
cargo run --release
```

## Step 4: Verify It's Working

### Health Check

```bash
curl http://localhost:9878/healthz
```

Expected response:

```
OK
```

### Readiness Check

```bash
curl http://localhost:9878/readyz
```

Expected response:

```
OK
```

### Metrics

```bash
curl http://localhost:9878/metrics
```

Example output (abbreviated):

```
# HELP verisure_alarm_armed_state Arm state: 0=disarmed, 1=armed_home, 2=armed_away
# TYPE verisure_alarm_armed_state gauge
verisure_alarm_armed_state{installation="123456789012"} 0

# HELP verisure_alarm_changed_timestamp_seconds Unix timestamp of last arm state change
# TYPE verisure_alarm_changed_timestamp_seconds gauge
verisure_alarm_changed_timestamp_seconds{installation="123456789012"} 1700000000

# HELP verisure_temperature_celsius Temperature in degrees Celsius
# TYPE verisure_temperature_celsius gauge
verisure_temperature_celsius{installation="123456789012",device_label="ABCD 1234",area="Living Room",device_type="SMOKE3"} 21.5
verisure_temperature_celsius{installation="123456789012",device_label="EFGH 5678",area="Bedroom",device_type="VOICEBOX"} 19.2

# HELP verisure_humidity_percent Relative humidity percentage
# TYPE verisure_humidity_percent gauge
verisure_humidity_percent{installation="123456789012",device_label="ABCD 1234",area="Living Room",device_type="SMOKE3"} 45.0

# HELP verisure_door_window_open 1=open, 0=closed
# TYPE verisure_door_window_open gauge
verisure_door_window_open{installation="123456789012",device_label="IJKL 9012",area="Front Door"} 0

# HELP verisure_lock_locked 1=locked, 0=unlocked
# TYPE verisure_lock_locked gauge
verisure_lock_locked{installation="123456789012",device_label="MNOP 3456",area="Front Door"} 1

# HELP verisure_broadband_connected 1 if broadband connected
# TYPE verisure_broadband_connected gauge
verisure_broadband_connected{installation="123456789012"} 1

# HELP verisure_scrape_duration_seconds Duration of the last scrape
# TYPE verisure_scrape_duration_seconds gauge
verisure_scrape_duration_seconds 1.234

# HELP verisure_scrape_success 1 if last scrape succeeded
# TYPE verisure_scrape_success gauge
verisure_scrape_success 1
```

## Troubleshooting

| Issue | Cause | Solution |
|-------|-------|----------|
| `401 Unauthorized` on startup | Wrong credentials | Double-check `VERISURE_USERNAME` and `VERISURE_PASSWORD`. Ensure you can log in to the Verisure app. |
| `403 Forbidden` | MFA is enabled | Disable MFA on the Verisure account. The exporter cannot handle MFA challenges. |
| No metrics returned | Scrape hasn't completed yet | Wait for the first poll interval (default 60s) to complete. Check logs for errors. |
| Connection refused | Exporter not running | Check `docker logs verisure-exporter` for startup errors. Verify port 9878 is not in use. |
| GIID auto-detection fails | Multiple installations or API issue | Set `VERISURE_GIID` explicitly as an environment variable. |
| Repeated auth errors in logs | Session expired + re-auth failing | The exporter automatically re-authenticates on 401. If this keeps failing, the password may have changed. |

## Next Steps

- **Kubernetes**: Deploy with Helm -- see the [Helm Deployment Guide](helm-deployment.md)
- **Prometheus**: Add a scrape config targeting the exporter on port 9878
- **Grafana**: Build dashboards using the exported metrics -- alarm state, temperatures, humidity, lock status, and more
- **Alerting**: Set up alerts for alarm state changes, temperature thresholds, door/window left open, motor jams, or broadband outages
