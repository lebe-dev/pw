# Monitoring

PW exposes a `/api/metrics` endpoint that returns all metrics in [Prometheus exposition format](https://prometheus.io/docs/instrumenting/exposition_formats/) (`text/plain; version=0.0.4`). All metrics are gauges. No external metrics library is used; the response is hand-serialised by the backend.

## 1. Metrics reference

| Metric | Description |
|---|---|
| `pw_up` | Always `1` while the process is running |
| `pw_build_info` | Always `1`. The running version is in the `version` label |
| `pw_uptime_seconds` | Seconds since the process started |
| `pw_redis_up` | `1` if the Redis SET+GET health check succeeded, `0` otherwise |
| `pw_redis_latency_seconds` | Round-trip time of the Redis health check in seconds. `NaN` when Redis is down |
| `pw_config_message_max_length` | Configured maximum message length |
| `pw_config_file_max_size_bytes` | Configured maximum file size in bytes |
| `pw_config_file_upload_enabled` | `1` if file upload is enabled, `0` otherwise |
| `pw_ip_limits_enabled` | `1` if IP-based limits are enabled, `0` otherwise |
| `pw_body_limit_bytes` | HTTP request body size limit in bytes |

## 2. Scraping with Docker Compose

In Docker Compose mode there is no nginx sidecar. The backend serves on port `8080` directly and `/api/metrics` is reachable without restriction.

```bash
curl http://localhost:8080/api/metrics
```

Add a scrape target to your Prometheus configuration:

```yaml
scrape_configs:
  - job_name: pw
    static_configs:
      - targets: ["localhost:8080"]
```

## 3. Scraping in Kubernetes

The Helm chart runs an nginx sidecar in the same pod. nginx listens on port `8080` and proxies to the backend on port `8081`. The sidecar explicitly blocks the metrics path:

```nginx
location = /api/metrics {
    deny all;
}
```

The main ClusterIP Service exposes port `8080` (the nginx port), so `/api/metrics` is not reachable through the Ingress. However, the Helm chart creates a separate `pw-metrics` Service (when `pw.metrics.enabled: true`, which is the default) that exposes the backend port `8081` for Prometheus scraping.

If you use Prometheus Operator, add a `ServiceMonitor`:

```yaml
apiVersion: monitoring.coreos.com/v1
kind: ServiceMonitor
metadata:
  name: pw
  labels:
    app.kubernetes.io/name: pw
spec:
  selector:
    matchLabels:
      app.kubernetes.io/component: pw-metrics
  endpoints:
    - port: metrics
      path: /api/metrics
      interval: 30s
```

Alternatively, add a manual scrape config targeting the metrics Service:

```yaml
scrape_configs:
  - job_name: pw
    kubernetes_sd_configs:
      - role: service
    relabel_configs:
      # Keep only the metrics service
      - source_labels: [__meta_kubernetes_service_label_app_kubernetes_io_component]
        regex: pw-metrics
        action: keep
      # Expose the namespace
      - source_labels: [__meta_kubernetes_namespace]
        target_label: namespace
      # Expose the release name
      - source_labels: [__meta_kubernetes_service_label_app_kubernetes_io_instance]
        target_label: instance
```

## 4. Alerting rules

The rules below are written as a Kubernetes `PrometheusRule` custom resource (Prometheus Operator). The same `groups` block is valid as a standalone Prometheus rules file.

```yaml
apiVersion: monitoring.coreos.com/v1
kind: PrometheusRule
metadata:
  name: pw-alerts
  labels:
    # If you use Prometheus Operator, add the label that your
    # PrometheusRule selector matches on. Example:
    #   prometheus: kube-prometheus
    app.kubernetes.io/name: pw
spec:
  groups:
    - name: pw
      interval: 30s
      rules:
        # Prometheus could not scrape the target at all.
        # This fires when the scrape itself fails — for example
        # the pod is gone or the port is unreachable.
        - alert: PwTargetDown
          expr: up{job="pw"} == 0
          for: 1m
          labels:
            severity: critical
          annotations:
            summary: "PW scrape target is down"
            description: >
              Prometheus cannot reach the /api/metrics endpoint
              for instance {{ $labels.instance }} in namespace
              {{ $labels.namespace }}.

        # The app is reachable but self-reports as unhealthy.
        # pw_up is currently always 1; this alert is a safety net
        # in case that logic changes.
        - alert: PwAppDown
          expr: pw_up == 0
          for: 1m
          labels:
            severity: critical
          annotations:
            summary: "PW application reports unhealthy"
            description: >
              pw_up is 0 for instance {{ $labels.instance }} in
              namespace {{ $labels.namespace }}.

        # Redis is not reachable from the backend.
        - alert: PwRedisDown
          expr: pw_redis_up == 0
          for: 1m
          labels:
            severity: critical
          annotations:
            summary: "Redis is down"
            description: >
              The Redis health check failed for instance
              {{ $labels.instance }} in namespace
              {{ $labels.namespace }}. Secrets cannot be stored or
              retrieved.

        # Redis is reachable but slow.  The default threshold is
        # 100 ms.  Adjust to match your environment.
        - alert: PwRedisLatencyHigh
          expr: pw_redis_latency_seconds > 0.1
          for: 2m
          labels:
            severity: warning
          annotations:
            summary: "Redis latency is high"
            description: >
              Redis round-trip latency is {{ $value | humanizeDuration }}
              for instance {{ $labels.instance }} in namespace
              {{ $labels.namespace }}. Threshold is 100 ms.

        # The process restarted recently (within the last 5 minutes).
        # A single restart is normal after a deploy; this alert
        # matters when it keeps firing.
        - alert: PwRestarted
          expr: pw_uptime_seconds < 300
          for: 1m
          labels:
            severity: info
          annotations:
            summary: "PW restarted recently"
            description: >
              Process uptime is {{ $value | humanizeDuration }} for
              instance {{ $labels.instance }} in namespace
              {{ $labels.namespace }}.
```

## Related

- [Security](SECURITY.md)
- [Installation — Docker](install/DOCKER.md)
- [Installation — Kubernetes](install/KUBERNETES.md)
- [Backend API](API.md)
