# Contrato Público

> English version of this README is available at [README.en.md](https://github.com/chicoferreira/contratopublico/blob/main/README.en.md)

O [contratopublico.pt](https://contratopublico.pt/) é um serviço de pesquisa de contratos públicos
realizados em Portugal que agrega dados do
[Portal BASE](https://www.base.gov.pt/base4)[^1], disponibilizando-os numa plataforma muito mais usável, rápida e intuitiva.

<img width="1334" height="834" alt="image" src="https://github.com/user-attachments/assets/90aac3d9-8959-474f-ae45-940be8ac5b53" />

[^1]: Sem qualquer afiliação com o Governo de Portugal ou o IMPIC.

O Portal BASE oficial apresenta [um desempenho muito fraco, mesmo para pesquisas simples de contratos](https://www.base.gov.pt/Base4/pt/pesquisa/?type=contratos&texto=Porto&tipo=0&tipocontrato=0&cpv=&aqinfo=&adjudicante=&adjudicataria=&sel_price=price_c1&desdeprecocontrato=&ateprecocontrato=&desdeprecoefectivo=&ateprecoefectivo=&desdeprazoexecucao=&ateprazoexecucao=&sel_date=date_c1&desdedatacontrato=&atedatacontrato=&desdedatapublicacao=&atedatapublicacao=&desdedatafecho=&atedatafecho=&pais=0&distrito=0&concelho=0), cada uma demorando frequentemente mais de 5 segundos, e [algumas consultas podem ultrapassar os 50 segundos](https://www.base.gov.pt/Base4/pt/pesquisa/?type=contratos&texto=&tipo=0&tipocontrato=0&cpv=&aqinfo=&adjudicante=Municipio+de+Santo+Tirso&adjudicataria=&sel_price=price_c1&desdeprecocontrato=&ateprecocontrato=&desdeprecoefectivo=&ateprecoefectivo=&desdeprazoexecucao=&ateprazoexecucao=&sel_date=date_c1&desdedatacontrato=&atedatacontrato=&desdedatapublicacao=&atedatapublicacao=&desdedatafecho=&atedatafecho=&pais=0&distrito=0&concelho=0).

Embora não seja uma comparação direta e exata, uma vez que a nossa recolha de dados apenas obtém informação superficial apresentada nos resultados de pesquisa do Portal BASE e não inclui campos como _localização_ ou _entidades concorrentes_ (ver [issue #28](https://github.com/chicoferreira/contratopublico/issues/28) para progresso), a diferença nos tempos de resposta continua a ser astronómica. A [mesma pesquisa simples](https://contratopublico.pt/?query=Porto) e [a pesquisa com os mesmos filtros](https://contratopublico.pt/?contracting=Municipio+do+Porto) no [contratopublico.pt](https://contratopublico.pt) devolvem resultados em apenas alguns milissegundos, tornando a pesquisa praticamente instantânea.

Está planeada disponibilização de informação mais abrangente e interativa sobre este tema de desempenho em [issue #10](https://github.com/chicoferreira/contratopublico/issues/10).

O [contratopublico.pt](https://contratopublico.pt) também proporciona uma experiência de pesquisa muito mais intuitiva e acessível, com funcionalidades de pesquisa automática, descrições detalhadas para termos técnicos menos familiares ao público geral e uma interface significativamente mais apelativa e funcional.

## Funcionalidades Planeadas

Atualmente, este projeto permite pesquisar contratos públicos com base em informações simples como título do contrato, entidade adjudicante, entidade adjudicatária, datas, entre outros.

Estão planeadas várias novas funcionalidades para tornar o projeto mais informativo. Entre elas, a integração de estatísticas interativas, como gráficos que mostram a despesa diária, mensal e anual com contratos, destaques com os contratos mais caros e quais as localizações/instituições com maior despesa.

Também está prevista a implementação de páginas dedicadas a cada contrato, com informação mais detalhada, evitando que o utilizador tenha de visitar o Portal BASE para obter mais dados.

Além disso, está planeado o acompanhamento de alterações aos contratos após a sua publicação e a criação de uma página para aceder a informações de cada entidade, mostrando o seu histórico de contratos e estatísticas relacionadas.

Para a lista completa de funcionalidades planeadas, consulte as [issues](https://github.com/chicoferreira/contratopublico/issues/). Contribuições são bem-vindas. Caso tenha sugestões ou novas ideias, crie uma _issue_ a descrevê-las.

## Estrutura do projeto

### Stack

- **Backend**: Rust com Axum
- **Motor de Pesquisa**: Meilisearch
- **Frontend**: SvelteKit + Tailwind + TypeScript
- **Monitorização**: Prometheus + Grafana

```
backend/                # Backend em Rust
  crates/
    api/                # API Axum-
    common/             # Tipos partilhados
    scraper/            # Scraper e CLI
frontend/               # Aplicação SvelteKit
docker/                 # Ficheiros Compose
monitoring/             # Prometheus + Grafana + k6
rpxy/                   # Configuração do proxy reverso
```

### _Scraping_

O serviço backend recolhe continuamente dados do Portal BASE usando a _crate_ `scraper` e adiciona novos contratos ao Meilisearch.

### Monitorização

O Prometheus e o Grafana com uma _dashboard_ simples estão incluídos em `docker/docker-compose.yml`, com as suas configurações em `monitoring/grafana`.

Um simples script de _benchmark_ com `k6` também está incluído em `monitoring/bench`.

## Início rápido (Docker)

**Pré-requisitos:** Docker & Docker Compose

1. Copie o ficheiro `.env.example` para `.env` dentro da pasta `docker/` e altere os valores conforme necessário.

2. Execute o _compose_:

```
docker compose -f docker/docker-compose.yml up -d
```

Isto irá iniciar:

- `meilisearch` (dados em `backend/data/meili_data`)
- `backend`
- `frontend`
- `prometheus` e `grafana`
- `rpxy` (proxy reverso)

Por predefinição, as portas não são expostas. Em produção, deve disponibilizar o seu próprio _proxy_ (por exemplo, `docker-compose-cftunnels.yml` inicia um Cloudflare Tunnel). Para utilização local pode:

- Executar os serviços localmente sem Docker (ver secção seguinte), ou:
  1. Adicionar `ports:` no `docker/docker-compose.yml` no serviço `rpxy`, expondo a porta 80.
  2. Alterar `server_name` de `contratopublico.pt` para `localhost` em `rpxy/config/config.toml`.

## Desenvolvimento local

Pode executar o Meilisearch em Docker, o backend com Cargo e o frontend com Bun/Node.

### 1. Meilisearch

Inicie o Meilisearch em Docker:

```
docker compose -f docker/docker-compose-meilisearch.yml up -d
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
- Iniciar um ciclo periódico de _scraping_
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

Consulte a licença de utilização em [LICENSE](https://github.com/chicoferreira/contratopublico/blob/main/LICENSE)
