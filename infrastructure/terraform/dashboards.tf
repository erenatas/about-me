resource "grafana_dashboard" "about-me-general" {
  provider = grafana

  config_json = templatefile("${path.module}/dashboards/about-me-general.json.tftpl", {
    prometheus_uid = grafana_data_source.prometheus-data-source.uid
    pyroscope_uid = grafana_data_source.pyroscope-data-source.uid
    loki_uid = grafana_data_source.loki-data-source.uid
    tempo_uid = grafana_data_source.tempo-data-source.uid
  })
}