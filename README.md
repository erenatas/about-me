# about-me

Personal page + monitoring stack (Leptos/Axum + LPGTM).

## Stack config (set in Mongo, visible in the UI)

| Field | Value | Why |
|---|---|---|
| `files_on_host` | `true` | Use `/home/peri/GitRepos/peripery-repos/about-me` directly, no clone |
| `run_build` | `true` | Build `app` service via local source on each deploy |
| `auto_pull` | `false` | Other services use pinned versions; no Hub to pull from |
| `run_directory` | `/home/peri/GitRepos/peripery-repos/about-me` | Komodo `cd`s here before `docker compose` |
| `file_paths` | `["docker-compose.yml"]` | The compose file to apply |
| `env_file_path` | `.env` | Passes `.env` via `--env-file` |
| `destroy_before_deploy` | `false` | Won't `compose down` first (preserves volumes) |
| `branch` | `master` | Only used for webhook branch matching |

## Deploy flow

```bash
cd /home/peri/GitRepos/peripery-repos/about-me
./deploy.sh           # git fetch + git pull --ff-only
```

Then in the Komodo UI: *Stacks → about-me → Deploy*.

What Komodo does:
1. `cd /home/peri/GitRepos/peripery-repos/about-me`
2. `docker compose -p about-me -f docker-compose.yml --env-file .env build app`
3. `docker compose -p about-me -f docker-compose.yml --env-file .env up -d`

First deploy takes ~10 min for the cargo-leptos build. Subsequent
deploys are faster thanks to the Docker layer cache.

## Local dev

Use `cargo leptos` directly. The dev compose was archived to
`archive/docker-compose.dev.yml` because it contained hardcoded
secrets (e.g. `POSTGRES_PASSWORD=postgres!t0il`) and exposed the
monitoring ports to 0.0.0.0; you don't want it accidentally deployed
in prod.

### Provision Grafana resources via Terraform

```bash
pushd infrastructure/terraform
cp .env.example .env   # fill in
terraform init --var-file .env
terraform plan  --var-file .env
terraform apply --var-file .env
```

## TODO
- Production deployment
- Roadmap, features
- Observability Stack
- README Documentation
- Better code/file structure
- IaC, AIO Deployment
