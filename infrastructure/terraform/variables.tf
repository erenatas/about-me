variable "GRAFANA_URL" {
    type = string
}

variable "PROMETHEUS_URL" {
    type = string
    default = "http://prometheus:9090"
}

variable "LOKI_URL" {
    type = string
    default = "http://loki:3100"
}

variable "TEMPO_URL" {
    type = string
    default = "http://tempo:4317"
}

variable "PYROSCOPE_URL" {
    type = string
    default = "http://pyroscope:4040"
}

variable "GRAFANA_USERNAME" {
    type = string
}

variable "GRAFANA_PASSWORD" {
    type = string
}