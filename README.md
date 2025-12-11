> English version of this README is available at [README.en.md](https://github.com/chicoferreira/contratopublico/blob/main/README.en.md)

# Contrato Público

O [contratopublico.pt](https://contratopublico.pt/) é um serviço de pesquisa de contratos públicos realizados em Portugal que agrega dados do [Portal BASE](https://www.base.gov.pt/base4)[^1], disponibilizando-os numa plataforma muito mais usável, rápida e intuitiva.

<img width="1334" height="834" alt="image" src="https://github.com/user-attachments/assets/c7fb7044-9466-41d0-ba49-7b57114bbe3d" />

[^1]: Sem qualquer afiliação com o Governo de Portugal ou o IMPIC.

O Portal BASE oficial apresenta [um desempenho muito fraco, mesmo para pesquisas simples de contratos](https://www.base.gov.pt/Base4/pt/pesquisa/?type=contratos&texto=Porto&tipo=0&tipocontrato=0&cpv=&aqinfo=&adjudicante=&adjudicataria=&sel_price=price_c1&desdeprecocontrato=&ateprecocontrato=&desdeprecoefectivo=&ateprecoefectivo=&desdeprazoexecucao=&ateprazoexecucao=&sel_date=date_c1&desdedatacontrato=&atedatacontrato=&desdedatapublicacao=&atedatapublicacao=&desdedatafecho=&atedatafecho=&pais=0&distrito=0&concelho=0), cada uma demorando frequentemente mais de 5 segundos, e [algumas consultas podem ultrapassar os 50 segundos](https://www.base.gov.pt/Base4/pt/pesquisa/?type=contratos&texto=&tipo=0&tipocontrato=0&cpv=&aqinfo=&adjudicante=Municipio+de+Santo+Tirso&adjudicataria=&sel_price=price_c1&desdeprecocontrato=&ateprecocontrato=&desdeprecoefectivo=&ateprecoefectivo=&desdeprazoexecucao=&ateprazoexecucao=&sel_date=date_c1&desdedatacontrato=&atedatacontrato=&desdedatapublicacao=&atedatapublicacao=&desdedatafecho=&atedatafecho=&pais=0&distrito=0&concelho=0).

A [mesma pesquisa simples](https://contratopublico.pt/?query=Porto) e [a pesquisa com os mesmos filtros](https://contratopublico.pt/?contracting=Municipio+de+Santo+Tirso) no [contratopublico.pt](https://contratopublico.pt) devolvem resultados em apenas alguns milissegundos, tornando a pesquisa praticamente instantânea.

O [contratopublico.pt](https://contratopublico.pt) também proporciona uma experiência de pesquisa muito mais intuitiva e acessível, com funcionalidades de pesquisa automática, descrições detalhadas para termos técnicos menos familiares ao público geral e uma interface significativamente mais apelativa e funcional.

<img width="1334" height="894" alt="image" src="https://github.com/user-attachments/assets/27396df4-1a0d-4a52-98d4-aed2bfa888e1" />

## Funcionalidades Planeadas

Atualmente, este projeto permite pesquisar contratos públicos com base em informações simples como título do contrato, entidade adjudicante, entidade adjudicatária, datas, entre outros, e consultar páginas dedicadas a cada contrato com informação detalhada.

Estão planeadas várias novas funcionalidades para tornar o projeto mais informativo. Entre elas, a integração de estatísticas interativas, como gráficos que mostram a despesa diária, mensal e anual com contratos, destaques com os contratos mais caros e quais as localizações/instituições com maior despesa.

Além disso, está planeado o acompanhamento de alterações aos contratos após a sua publicação e a criação de uma página para aceder a informações de cada entidade, mostrando o seu histórico de contratos e estatísticas relacionadas.

Para a lista completa de funcionalidades planeadas, consulta as [issues](https://github.com/chicoferreira/contratopublico/issues/). Contribuições são bem-vindas. Caso tenhas sugestões ou novas ideias, cria uma _issue_ a descrevê-las, ou se te sentires à vontade para a implementar, não hesites em mandar um PR.

## Estrutura do projeto

### Stack

- **Backend**: Rust com Axum
- **Motor de Pesquisa**: Meilisearch
- **Base de Dados**: Postgres
- **Frontend**: SvelteKit + Tailwind + shadcn + TypeScript
- **Monitorização**: Prometheus + Grafana

```
backend/                # Backend em Rust
  crates/
    api/                # API Axum
    common/             # Tipos partilhados
    scraper/            # Scraper e CLI
frontend/               # Aplicação SvelteKit
docker/                 # Ficheiros Compose
monitoring/             # Prometheus + Grafana + k6
rpxy/                   # Configuração do proxy reverso
```

### _Scraping_

O serviço backend recolhe continuamente dados do Portal BASE usando a _crate_ `scraper` e adiciona novos contratos ao Meilisearch (para pesquisa de contratos) e à base de dados Postgres (para consulta de informação detalhada de contratos).

### Monitorização

O Prometheus e o Grafana com uma _dashboard_ simples estão incluídos em `docker/docker-compose.yml`, com as suas configurações em `monitoring/grafana`.

Um simples script de _benchmark_ com `k6` também está incluído em `monitoring/bench`.

## Início rápido (Docker)

**Pré-requisitos:** Docker & Docker Compose

1. Copia o ficheiro `.env.example` para `.env` dentro da pasta `docker/` e altere os valores conforme necessário.

2. Executa o _compose_:

```
docker compose -f docker/compose.yml up -d
```

Isto irá iniciar:

- `meilisearch`
- `postgres`
- `backend`
- `frontend`
- `prometheus` e `grafana`
- `rpxy` (proxy reverso)

Por predefinição, as portas não são expostas. Em produção, deves disponibilizar o teu próprio _proxy_ (por exemplo, o `compose-cftunnels.yml` inicia um Cloudflare Tunnel). Para utilização local podes:

- Executar os serviços localmente sem Docker (ver secção seguinte), ou:
  1. Adicionar `ports:` no `docker/compose.yml` no serviço `rpxy`, expondo a porta 80.
  2. Alterar `server_name` de `contratopublico.pt` para `localhost` em `rpxy/config/config.toml`.

## Desenvolvimento local

Podes executar o Meilisearch e o Postgres no Docker, o backend com Cargo e o frontend com Bun/Node.

### 1. Meilisearch

Inicia o Meilisearch e o Postgres no Docker:

```
docker compose -f docker/compose-meilisearch.yml up -d
```

### 2. Backend (Rust)

**Requisitos:** Rust

Consulte `backend/src/api/src/main.rs` para variáveis de ambiente.

Execute:

```
cd backend
cargo run --release --bin backend
```

O backend irá (por predefinição):

- Preparar as definições do Meilisearch
- Executar migrações no Postgres
- Iniciar um ciclo periódico de _scraping_ (usa a _flag_ `--no-scraper` para não iniciar)
- Expor a API em `:3000` e métricas em `:3001/metrics`

### 3. Frontend (SvelteKit)

**Requisitos:** Bun (recomendado) ou Node 20+

Execute:

```
cd frontend
bun install
bun run dev
```

Será apresentada a porta onde o frontend foi exposto.

## Licença

Consulta a licença de utilização em [LICENSE](https://github.com/chicoferreira/contratopublico/blob/main/LICENSE)
