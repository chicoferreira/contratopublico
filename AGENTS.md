# AGENTS.md

[contratopublico.pt](https://contratopublico.pt/) is a Portuguese public-contract search service. A [Rust scraper](backend/crates/scraper/) collects data from [Portal BASE](https://www.base.gov.pt/base4) into Postgres and Meilisearch; an [Axum API](backend/crates/api/) serves a [SvelteKit frontend](frontend/). See [README.en.md](README.en.md) for setup and project layout.

## Repository rules

- Keep [`frontend/src/lib/types/api.ts`](frontend/src/lib/types/api.ts) in sync with backend API types.
- Update the [privacy policy](frontend/src/routes/privacy/+page.md) when changing user data collection or logging.
- Document notable user-facing changes in the [changelog](frontend/src/routes/changelog/+page.md) in a separate commit that references the implementation SHA.
- Write code, identifiers, and commit messages in English. Write site text in Portuguese (pt-PT). Avoid explanatory code comments.
- Use a short imperative commit message without a prefix.

## Checks

- Backend (from [`backend/`](backend/)): run `cargo fmt`, `cargo clippy --workspace`, and `cargo test`. `#[sqlx::test]` requires Postgres running and `DATABASE_URL` environment variable.
- Frontend (from [`frontend/`](frontend/)): format changed files with `bunx prettier --write <files>`, then run `bun run check` and `bun run build`.
