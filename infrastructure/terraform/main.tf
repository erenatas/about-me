terraform { 
  cloud { 
    
    organization = "AtasHome" 

    workspaces { 
      name = "about-me-infrastructure" 
    } 
  } 
  
  required_providers {
    grafana = {
      source  = "grafana/grafana"
      version = ">= 2.9.0"
    }
  }
}


provider "grafana" {
  url  = var.GRAFANA_URL
  auth = "${var.GRAFANA_USERNAME}:${var.GRAFANA_PASSWORD}"
}
