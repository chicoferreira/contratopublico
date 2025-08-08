> Portuguese version of this README is available at [README.md](https://github.com/chicoferreira/contratopublico/blob/main/README.md)

# Contrato Público (Public Contracts)

[contratopublico.pt](https://contratopublico.pt/) is a search service for public contracts executed in Portugal. It aggregates data from the official database [Portal BASE](https://www.base.gov.pt/base4)[^1], making it available on a platform that is far more usable, faster, and intuitive.

<img width="1334" height="834" alt="image" src="https://github.com/user-attachments/assets/90aac3d9-8959-474f-ae45-940be8ac5b53" />

[^1]: Not affiliated in any way with the Government of Portugal or IMPIC.

The official Portal BASE [has very poor performance, even for simple contract searches](https://www.base.gov.pt/Base4/pt/pesquisa/?type=contratos&texto=Porto&tipo=0&tipocontrato=0&cpv=&aqinfo=&adjudicante=&adjudicataria=&sel_price=price_c1&desdeprecocontrato=&ateprecocontrato=&desdeprecoefectivo=&ateprecoefectivo=&desdeprazoexecucao=&ateprazoexecucao=&sel_date=date_c1&desdedatacontrato=&atedatacontrato=&desdedatapublicacao=&atedatapublicacao=&desdedatafecho=&atedatafecho=&pais=0&distrito=0&concelho=0), often taking more than 5 seconds per search, and [some queries can take over 50 seconds](https://www.base.gov.pt/Base4/pt/pesquisa/?type=contratos&texto=&tipo=0&tipocontrato=0&cpv=&aqinfo=&adjudicante=Municipio+de+Santo+Tirso&adjudicataria=&sel_price=price_c1&desdeprecocontrato=&ateprecocontrato=&desdeprecoefectivo=&ateprecoefectivo=&desdeprazoexecucao=&ateprazoexecucao=&sel_date=date_c1&desdedatacontrato=&atedatacontrato=&desdedatapublicacao=&atedatapublicacao=&desdedatafecho=&atedatafecho=&pais=0&distrito=0&concelho=0).

Although this is not a direct and exact comparison, since our data collection only retrieves the superficial information shown in the Portal BASE search results and does not include fields like _location_ or _competing entities_ (see [issue #28](https://github.com/chicoferreira/contratopublico/issues/28) for progress), the difference in response times remains astronomical. The [same simple search](https://contratopublico.pt/?query=Porto) and [search with the same filters](https://contratopublico.pt/?contracting=Municipio+de+Santo+Tirso) on [contratopublico.pt](https://contratopublico.pt) return results in just a few milliseconds, making the search practically instantaneous.

More extensive and interactive information about this performance topic is planned in [issue #10](https://github.com/chicoferreira/contratopublico/issues/10).

[contratopublico.pt](https://contratopublico.pt) also offers a much more intuitive and accessible search experience, with features like automatic search, detailed descriptions for technical terms that may be unfamiliar to the general public, and a significantly more appealing and functional interface.

## Planned Features

Currently, this project allows you to search public contracts based on simple information such as contract title, contracting authority, contractor, dates, among others.

Several new features are planned to make the project more informative. These include the integration of interactive statistics, such as charts showing daily, monthly, and yearly spending on contracts, highlights of the most expensive contracts, and which locations/institutions have the highest spending.

There are also plans to implement dedicated pages for each contract, with more detailed information, avoiding the need for the user to visit the Portal BASE to get more data.

In addition, there are plans to track changes to contracts after their publication and to create a page with information on each entity, showing its contract history and related statistics.

For the full list of planned features, check the [issues](https://github.com/chicoferreira/contratopublico/issues/). Contributions are welcome. If you have suggestions or new ideas, create an _issue_ describing them.

## Project Structure

### Stack

- **Backend**: Rust with Axum
- **Search Engine**: Meilisearch
- **Frontend**: SvelteKit + Tailwind + TypeScript
- **Monitoring**: Prometheus + Grafana

```
backend/                # Rust backend
  crates/
    api/                # Axum API
    common/             # Shared types
    scraper/            # Scraper and CLI
frontend/               # SvelteKit application
docker/                 # Compose files
monitoring/             # Prometheus + Grafana + k6
rpxy/                   # Reverse proxy configuration
```

### Scraping

The backend service continuously collects data from the Portal BASE using the `scraper` crate and adds new contracts to Meilisearch.

### Monitoring

Prometheus and Grafana with a simple dashboard are included in `docker/docker-compose.yml`, with their configurations in `monitoring/grafana`.

A simple `k6` benchmark script is also included in `monitoring/bench`.

## Quick Start (Docker)

**Prerequisites:** Docker & Docker Compose

1. Copy the `.env.example` file to `.env` inside the `docker/` folder and change the values as needed.

2. Run the compose:

```
docker compose -f docker/docker-compose.yml up -d
```

This will start:

- `meilisearch` (data in `backend/data/meili_data`)
- `backend`
- `frontend`
- `prometheus` and `grafana`
- `rpxy` (reverse proxy)

By default, the ports are not exposed. In production, you should provide your own proxy (for example, `docker-compose-cftunnels.yml` starts a Cloudflare Tunnel). For local use, you can:

- Run the services locally without Docker (see next section), or:
  1. Add `ports:` to `rpxy` in `docker/docker-compose.yml` to expose port 80.
  2. Change `server_name` from `contratopublico.pt` to `localhost` in `rpxy/config/config.toml`.

## Local Development

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

- Prepare Meilisearch settings
- Start a periodic scraping cycle
- Expose the API on `:3000` and metrics on `:3001/metrics`

### 3. Frontend (SvelteKit)

**Requirements:** Bun (recommended) or Node 20+

Run:

```
cd frontend
bun install
bun run dev
```

The port where the frontend is exposed will be shown.

## License

See the usage license in [LICENSE](https://github.com/chicoferreira/contratopublico/blob/main/LICENSE)
