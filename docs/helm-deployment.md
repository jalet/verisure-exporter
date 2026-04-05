# Helm Deployment Guide

This guide covers deploying the Verisure Exporter to Kubernetes using the Helm chart.

## Prerequisites

- **Helm 3.x** installed
- A **Kubernetes cluster** (1.24+)
- A **monitoring namespace** (or your preferred namespace)
- **Verisure credentials** (username, password, and optionally GIID)
- For ExternalSecrets + OpenBao: see the [optional section below](#optional-externalsecrets-with-openbao)

## Step 1: Create Kubernetes Secret

Create a secret containing your Verisure credentials:

```bash
kubectl create namespace monitoring --dry-run=client -o yaml | kubectl apply -f -

kubectl create secret generic verisure-credentials \
  --namespace monitoring \
  --from-literal=username=your@email.com \
  --from-literal=password=yourpassword \
  --from-literal=giid=123456789012
```

The `giid` field is optional. If omitted, the exporter will auto-detect the installation GIID on startup.

## Step 2: Create values-prod.yaml

Create a values file for your deployment:

```yaml
# values-prod.yaml
replicaCount: 1

image:
  repository: ghcr.io/jalet/verisure-exporter
  tag: ""  # defaults to chart appVersion

verisure:
  credentialsSecretRef: verisure-credentials

exporter:
  pollInterval: 60
  logLevel: info

service:
  type: ClusterIP
  port: 9878

serviceMonitor:
  enabled: true
  interval: 60s
  namespace: monitoring

prometheusRule:
  enabled: true

networkPolicy:
  enabled: true

resources:
  requests:
    cpu: 10m
    memory: 32Mi
  limits:
    memory: 64Mi

serviceAccount:
  create: true
```

## Step 3: Install the Chart

```bash
helm install verisure-exporter \
  oci://ghcr.io/jalet/verisure-exporter/verisure-exporter \
  --namespace monitoring \
  --values values-prod.yaml
```

## Step 4: Verify the Deployment

```bash
# Check pod status
kubectl get pods -n monitoring -l app.kubernetes.io/name=verisure-exporter

# Check logs
kubectl logs -n monitoring -l app.kubernetes.io/name=verisure-exporter

# Port-forward to test locally
kubectl port-forward -n monitoring svc/verisure-exporter 9878:9878

# In another terminal
curl http://localhost:9878/healthz
curl http://localhost:9878/readyz
curl http://localhost:9878/metrics
```

## Step 5: ServiceMonitor

If you set `serviceMonitor.enabled: true` in your values file, a `ServiceMonitor` resource is created automatically. This tells Prometheus to scrape the exporter.

Verify the ServiceMonitor was created:

```bash
kubectl get servicemonitor -n monitoring verisure-exporter
```

If your Prometheus Operator is in a different namespace, you may need to set `serviceMonitor.namespace` accordingly.

## Step 6: Alerting Rules

If you set `prometheusRule.enabled: true`, a `PrometheusRule` resource is created with default alerting rules.

Verify:

```bash
kubectl get prometheusrule -n monitoring verisure-exporter
```

Example alerts you might configure:

```yaml
prometheusRule:
  enabled: true
  rules:
    - alert: VerisureExporterDown
      expr: up{job="verisure-exporter"} == 0
      for: 5m
      labels:
        severity: critical
      annotations:
        summary: "Verisure exporter is down"

    - alert: VerisureScrapeFailure
      expr: verisure_scrape_success == 0
      for: 10m
      labels:
        severity: warning
      annotations:
        summary: "Verisure scrape has been failing for 10 minutes"

    - alert: VerisureBroadbandDown
      expr: verisure_broadband_connected == 0
      for: 5m
      labels:
        severity: warning
      annotations:
        summary: "Verisure broadband is disconnected"

    - alert: VerisureLockMotorJam
      expr: verisure_lock_motor_jam == 1
      for: 1m
      labels:
        severity: critical
      annotations:
        summary: "Smart lock motor jam detected"
```

## Step 7: Network Policies

The Helm chart includes optional network policies (both standard Kubernetes `NetworkPolicy` and Cilium `CiliumNetworkPolicy`). Enable them in your values:

```yaml
networkPolicy:
  enabled: true

ciliumNetworkPolicy:
  enabled: true
```

These restrict traffic so that only Prometheus can scrape the exporter.

## Step 8: Verify Metrics in Prometheus

Open Prometheus and run a query:

```promql
verisure_alarm_armed_state
```

You should see the current alarm state for your installation.

## Step 9: Upgrade and Rollback

### Upgrade

```bash
helm upgrade verisure-exporter \
  oci://ghcr.io/jalet/verisure-exporter/verisure-exporter \
  --namespace monitoring \
  --values values-prod.yaml
```

### Rollback

```bash
# List revisions
helm history verisure-exporter -n monitoring

# Rollback to a specific revision
helm rollback verisure-exporter 1 -n monitoring
```

## (Optional) ExternalSecrets with OpenBao

For organizations using [OpenBao](https://openbao.org/) as a secrets backend, the `deploy/` directory contains an `ExternalSecret` resource that syncs Verisure credentials from OpenBao into Kubernetes.

### Prerequisites

- [External Secrets Operator](https://external-secrets.io/) installed in your cluster
- A `ClusterSecretStore` named `openbao` configured
- Credentials stored in OpenBao at `secret/data/verisure` with keys: `username`, `password`, `giid`

### Apply

```bash
kubectl apply -f deploy/externalsecret.yaml
```

This creates a Kubernetes secret that the Helm chart (or raw deployment manifests) can reference. See `deploy/` for the full set of raw manifests.

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Pod in CrashLoopBackOff | Check logs: `kubectl logs -n monitoring -l app.kubernetes.io/name=verisure-exporter`. Usually caused by wrong credentials or MFA being enabled. |
| ServiceMonitor not picked up | Verify the Prometheus Operator is watching the monitoring namespace. Check labels match. |
| No metrics in Prometheus | Port-forward to the exporter and check `/metrics` directly. Verify the ServiceMonitor selectors. |
| Secret not found | Ensure the secret name in values matches what you created. Check namespace. |
| ExternalSecret not syncing | Check the ExternalSecret status: `kubectl get externalsecret -n monitoring`. Verify the ClusterSecretStore is healthy. |
| Network policy blocking scrapes | Ensure Prometheus pods match the network policy's ingress selectors. Check pod labels. |

## Configuration Reference

| Value | Default | Description |
|-------|---------|-------------|
| `replicaCount` | `1` | Number of replicas |
| `image.repository` | `ghcr.io/jalet/verisure-exporter` | Container image repository |
| `image.tag` | `""` (appVersion) | Container image tag |
| `image.pullPolicy` | `IfNotPresent` | Image pull policy |
| `verisure.credentialsSecretRef` | `""` | Name of the Kubernetes secret with credentials |
| `exporter.pollInterval` | `60` | Seconds between API polls |
| `exporter.logLevel` | `info` | Log level |
| `exporter.listenAddress` | `0.0.0.0:9878` | HTTP listen address |
| `exporter.metricsPath` | `/metrics` | Metrics endpoint path |
| `service.type` | `ClusterIP` | Kubernetes service type |
| `service.port` | `9878` | Service port |
| `serviceMonitor.enabled` | `false` | Create a ServiceMonitor resource |
| `serviceMonitor.interval` | `60s` | Scrape interval |
| `serviceMonitor.namespace` | `""` | ServiceMonitor namespace |
| `prometheusRule.enabled` | `false` | Create a PrometheusRule resource |
| `networkPolicy.enabled` | `false` | Create a NetworkPolicy |
| `ciliumNetworkPolicy.enabled` | `false` | Create a CiliumNetworkPolicy |
| `serviceAccount.create` | `true` | Create a service account |
| `resources.requests.cpu` | `10m` | CPU request |
| `resources.requests.memory` | `32Mi` | Memory request |
| `resources.limits.memory` | `64Mi` | Memory limit |
