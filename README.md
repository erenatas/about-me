# about-me
Welcome to my playground. 
This repo contains code that is used to host erenatas.com

## Get started:
```
docker compose up
```

Setup nginx proxy manager at localhost:5481, afterwards it should be accessible as desired. Or port forward PORT 8080.

## Pillars of observability
This project makes use of LPGTM setup for showcasing 4 pillars of observability. Once you log into Grafana as admin (default password is admin, which will require you to replace the password). You can setup Grafana by using Terraform:

### Provision Grafana resources via Terraform:
First, go to terraform directory:

```bash
pushd infrastructure/terraform
```

and create `.env` file:
```bash
cp .env.example .env
```
and replace variables with what is needed.

Afterwards, you can run terraform CLI with following commands:
```bash
terraform init --var-file .env
terraform plan --var-file .env
terraform apply --var-file .env
```

## TODO
- Testing
- Roadmap, features
- Observability Stack
- README Documentation
- Better code/file structure
- IaC, AIO Deployment