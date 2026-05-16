ARG RUST_VERSION=1.85
ARG DEBIAN_RELEASE=bookworm

FROM rust:${RUST_VERSION}-slim-${DEBIAN_RELEASE} AS builder
WORKDIR /build

RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates curl pkg-config \
 && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
RUN mkdir src \
 && echo 'fn main() {}' > src/main.rs \
 && cargo build --release --locked \
 && rm -rf src target/release/portfolio* target/release/deps/portfolio*

COPY . .
RUN scripts/tailwind.sh -i styles/main.css -o public/css/main.css --minify
RUN cargo build --release --locked


FROM debian:${DEBIAN_RELEASE}-slim AS runtime
ARG APP_UID=10001
ARG APP_GID=10001

RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates curl tini \
 && rm -rf /var/lib/apt/lists/* \
 && groupadd --system --gid ${APP_GID} portfolio \
 && useradd --system --uid ${APP_UID} --gid ${APP_GID} --no-create-home --shell /usr/sbin/nologin portfolio

WORKDIR /app
COPY --from=builder --chown=portfolio:portfolio /build/target/release/portfolio /app/portfolio
COPY --from=builder --chown=portfolio:portfolio /build/public /app/public
COPY --from=builder --chown=portfolio:portfolio /build/i18n   /app/i18n

ENV PORTFOLIO_BIND=0.0.0.0:3000 \
    PORTFOLIO_LOG=info,portfolio=info,tower_http=info \
    RUST_BACKTRACE=1

USER portfolio
EXPOSE 3000

ENTRYPOINT ["/usr/bin/tini", "--", "/app/portfolio"]
