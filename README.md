# portfolio

Server-side rendered personal portfolio built in Rust with Axum, Maud and Tailwind CSS v4.

## How It Works

The server renders every page on each request from a typed JSON catalogue under `i18n/<locale>.json` and a Tailwind-compiled stylesheet. Locales are routed under `/<locale>/<path>` with the root redirecting to the configured default. SEO surfaces (sitemap, robots, site.webmanifest, hreflang, OpenGraph, JSON-LD) are emitted server-side per locale. The contact form posts to `/api/contact`, which validates input, rate-limits per IP, and delivers via SMTP through lettre.

## Quick Start

```bash
cp .env.example .env
make css
make run
```

Visit http://localhost:3000.

### Production

Single host with Docker:

```bash
docker compose up -d --build
```

Kubernetes via the bundled Helm chart:

```bash
helm install portfolio oci://ghcr.io/b1e90ff/charts/portfolio-service \
  -f helm/values-production.yaml
```

Both ship the same multi-stage image: non-root uid 10001, read-only rootfs, all caps dropped.

### Adding a Locale

Drop a new catalogue at `i18n/<locale>.json` mirroring the shape of `en-US.json`, then add the locale code to `PORTFOLIO_LOCALES`.

## Helm Chart

The `portfolio-service` Helm chart lives under `helm/` so the chart version, `appVersion` and the deployed image tag are bumped in lockstep with the Rust binary by a single `semantic-release` run. Each release publishes:

- `ghcr.io/b1e90ff/portfolio:<version>` — application image
- `ghcr.io/b1e90ff/charts/portfolio-service:<version>` — Helm chart

The chart wraps the generic `web-service` and `istio-sidecar-configurator` charts from `oci://ghcr.io/b1e90ff/charts`. Environment-specific overrides live in `helm/values-production.yaml`. CI runs `helm lint --strict` and renders both value files whenever anything under `helm/**` changes.

## Routes

| Path | Purpose |
|---|---|
| `/` | Redirect to `/<default-locale>` |
| `/<locale>` | Home |
| `/<locale>/about` | About |
| `/<locale>/projects` | Project list, client-side filterable |
| `/<locale>/projects/<id>` | Project detail |
| `/<locale>/contact` | Contact form |
| `/<locale>/privacy` · `/<locale>/impressum` | Legal pages |
| `POST /api/contact` | JSON submission, lettre SMTP |
| `GET /api/health` · `/healthz` | Liveness |
| `/sitemap.xml` · `/robots.txt` · `/site.webmanifest` | SEO endpoints |

## Configuration

| Variable | Required | Default | Description |
|---|---|---|---|
| `PORTFOLIO_BIND` | no | `0.0.0.0:3000` | Listen address |
| `PORTFOLIO_BASE_URL` | no | `http://localhost:3000` | Canonical URL used for OG, sitemap, hreflang |
| `PORTFOLIO_DEFAULT_LOCALE` | no | `en-US` | Redirect target for `/` |
| `PORTFOLIO_LOCALES` | no | `en-US,de-DE` | Comma-separated catalogues to load |
| `PORTFOLIO_LOG` | no | `info,portfolio=debug,tower_http=info` | `tracing_subscriber` filter |
| `SMTP_HOST` | for contact form | — | If unset, `/api/contact` returns 503 |
| `SMTP_PORT` | no | `587` | |
| `SMTP_USERNAME` | for contact form | — | |
| `SMTP_PASSWORD` | for contact form | — | |
| `SMTP_FROM` | no | `SMTP_USERNAME` | Sender mailbox |
| `SMTP_TO` | no | `SMTP_USERNAME` | Recipient mailbox |
| `SMTP_USE_STARTTLS` | no | `true` | Set `false` for implicit TLS on port 465 |

## Quality Gates

```bash
make check    # fmt --check + clippy -D warnings + tests
```

CI on every push and PR runs fmt, clippy, test, release build, cargo-audit, and a Docker build.
