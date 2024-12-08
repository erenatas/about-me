resource "grafana_data_source" "prometheus-data-source" {
  type = "prometheus"
  name = "Prometheus"
  url  = var.PROMETHEUS_URL
}

resource "grafana_data_source" "pyroscope-data-source" {
  type = "grafana-pyroscope-datasource"
  name = "grafana-pyroscope-datasource"
  url  = var.PYROSCOPE_URL
}

resource "grafana_data_source" "loki-data-source" {
  type = "loki"
  name = "Loki"  # Fixed duplicate name
  url  = var.LOKI_URL
}

resource "grafana_data_source" "tempo-data-source" {
  type = "tempo"
  name = "Tempo"  # Fixed duplicate name
  url  = var.TEMPO_URL
  json_data_encoded = jsonencode({
    "tracesToLogsV2": {
        "customQuery": false,
        "datasourceUid": grafana_data_source.loki-data-source.uid
    },
    "tracesToMetrics": {
        "datasourceUid": grafana_data_source.prometheus-data-source.uid
    },
    "tracesToProfiles": {
        "datasourceUid": grafana_data_source.pyroscope-data-source.uid
    }
  })
}
