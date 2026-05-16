# portfolio

Personal portfolio of Niklas Tat — Rust SSR with Axum, Maud, Tailwind v4.

## Run

```
make css      # build the stylesheet
make run      # debug server on :3000
```

## Production

```
docker compose up -d --build
```

Configurable via env (see `.env.example`).

## Tests

```
make check    # fmt + clippy + tests
```
