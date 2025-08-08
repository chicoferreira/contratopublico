# Contrato Público

[contratopublico.pt](https://contratopublico.pt/) is a search service for public contracts
executed in Portugal that aggregates data from the
official [Portal BASE](https://www.base.gov.pt/base4)[^1] into a much more usable, faster, and intuitive platform.

<img width="1334" height="834" alt="image" src="https://github.com/user-attachments/assets/90aac3d9-8959-474f-ae45-940be8ac5b53" />

[^1]: Not affiliated with Governo de Portugal or IMPIC.

## Performance

The official Portal BASE demonstrates [very poor performance for even simple contract searches](https://www.base.gov.pt/Base4/pt/pesquisa/?type=contratos&texto=Porto&tipo=0&tipocontrato=0&cpv=&aqinfo=&adjudicante=&adjudicataria=&sel_price=price_c1&desdeprecocontrato=&ateprecocontrato=&desdeprecoefectivo=&ateprecoefectivo=&desdeprazoexecucao=&ateprazoexecucao=&sel_date=date_c1&desdedatacontrato=&atedatacontrato=&desdedatapublicacao=&atedatapublicacao=&desdedatafecho=&atedatafecho=&pais=0&distrito=0&concelho=0), often taking more than 5 seconds, and [some queries can exceed 50 seconds](https://www.base.gov.pt/Base4/pt/pesquisa/?type=contratos&texto=&tipo=0&tipocontrato=0&cpv=&aqinfo=&adjudicante=Municipio+de+Santo+Tirso&adjudicataria=&sel_price=price_c1&desdeprecocontrato=&ateprecocontrato=&desdeprecoefectivo=&ateprecoefectivo=&desdeprazoexecucao=&ateprazoexecucao=&sel_date=date_c1&desdedatacontrato=&atedatacontrato=&desdedatapublicacao=&atedatapublicacao=&desdedatafecho=&atedatafecho=&pais=0&distrito=0&concelho=0).

While not a direct one-to-one comparison, since our scraping only collects the surface-level information displayed in Portal BASE search results and does not include fields like _location_ or _competitor entities_ (check [issue #28](https://github.com/chicoferreira/contratopublico/issues/28) for progress in this), the difference in response times is still astronomical. The [same simple query](https://contratopublico.pt/?query=Porto) and [the query with the same filters](https://contratopublico.pt/?contracting=Municipio+do+Porto) on [contratopublico.pt](https://contratopublico.pt) return the results in just a few milliseconds, making the search practically instantaneous.

More comprehensive and interactive information on this topic is planned in [issue #10](https://github.com/chicoferreira/contratopublico/issues/10).

## Planned Features

Currently, this project allows searching for public contracts using basic information such as contract title, contracting entity, contracted entity, and dates.

Many new features are planned to make the project more informative and user-friendly. Some include integrating interactive statistics, such as charts and graphs that show daily contract spending, highlight the most expensive contracts, and display which locations have the highest spending.

There are also plans to implement pages for contract views with more detailed information so users do not need to visit Portal BASE for further details.

Additionally, tracking contract modifications made after publication and offering pages for each entity, showing their contract history and related statistics, are planned.

For a complete list of planned features, check out the [issues tab](https://github.com/chicoferreira/contratopublico/issues/). Contributions are welcome. If you have suggestions or new ideas, please create a new issue describing them.

## Project structure

### Stack
- **Backend**: Rust with Axum
- **Search Engine**: Meilisearch
- **Frontend**: SvelteKit + Tailwind + TypeScript
- **Monitoring**: Prometheus + Grafana

```
backend/                # Rust backend
  crates/
    api/                # Axum API (search, statistics, metrics)
    common/             # Shared types (Contract, Currency)
    scraper/            # Scraper and CLI (scrape/import)
frontend/               # SvelteKit app
docker/                 # Compose files (stack, dev Meilisearch, Cloudflare tunnel)
monitoring/             # Prometheus + Grafana + k6
rpxy/                   # Reverse proxy configuration
```
### Scraping

The running backend service continuously scrapes Portal BASE using the `scraper` crate and adds new contracts into Meilisearch. The saved pages are tracked in `SAVED_PAGES_PATH`.

### Monitoring

Prometheus and Grafana are included in `docker/docker-compose.yml` with dashboards and configurations under `monitoring/grafana`. Set `GRAFANA_ADMIN_PASSWORD` in `.env`.

A simple `k6` benchmark script is also provided in `monitoring/bench`.

## Quick start (Docker)

**Prerequisites:** Docker & Docker Compose

1. Copy the `.env.example` into `.env` inside `docker/` and change the values as needed.

2. Run the compose:

```
docker compose -f docker/docker-compose.yml up -d
```

This brings up:

- `meilisearch` (data in `backend/data/meili_data`)
- `backend`
- `frontend`
- `prometheus` and `grafana`
- `rpxy` reverse proxy

By default, ports are not published. In production deploy behind your own proxy (for example `docker-compose-cftunnels.yml` brings up Cloudflare Tunnels). For local exploration you can either:

- Run services locally without Docker (see next section), or:
  1. Add `ports:` mappings to `docker/docker-compose.yml` for `rpxy` exposing port 80.
  2. Change `server_name` from `contratopublico.pt` to `localhost` in `rpxy/config/config.toml`.

## Local development

You can run Meilisearch in Docker, the backend with Cargo, and the frontend with Bun/Node.

### 1. Meilisearch

Start Meilisearch in Docker:

```
docker compose -f docker/docker-compose-meilisearch.yml up -d
```

### 2. Backend (Rust)

**Requirements:** Rust

Check `backend/src/api/src/main.rs` for environment variables.

Run:

```
cd backend
cargo run --release --bin backend
```

The backend will (by default):

- Prepare Meilisearch index settings
- Spawn a periodic scraper loop
- Expose API on `:3000` and metrics on `:3001/metrics`

### 3. Frontend (SvelteKit)

**Requirements:** Bun (recommended) or Node 20+

Run:

```
cd frontend
bun install
bun run dev
```
