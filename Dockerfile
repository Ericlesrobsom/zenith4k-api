# syntax=docker/dockerfile:1
#
# Imagem do `zenith4k-serve` (API REST local do painel merger).
#
# O servidor usa `include_str!` para dois arquivos fora do crate:
#   - docs/openapi.yaml
#   - docs/openapi.json
# Ambos PRECISAM ser copiados antes do `cargo build`, senao a compilacao falha.

FROM rust:1-slim-bookworm AS builder
WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Manifestos primeiro (cache de dependencias).
COPY Cargo.toml Cargo.lock ./
COPY crates/merger-core/Cargo.toml crates/merger-core/Cargo.toml
COPY crates/merger-server/Cargo.toml crates/merger-server/Cargo.toml
COPY crates/merger-cli/Cargo.toml crates/merger-cli/Cargo.toml

# Fontes dos membros do workspace.
COPY crates/merger-core/src crates/merger-core/src
COPY crates/merger-server/src crates/merger-server/src
COPY crates/merger-cli/src crates/merger-cli/src

# Embutidos via include_str! (ver crates/merger-server/src/routes.rs).
COPY docs/openapi.yaml docs/openapi.yaml
COPY docs/openapi.json docs/openapi.json

RUN cargo build --release -p merger-server --bin zenith4k-serve

FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --create-home --uid 10001 zenith

WORKDIR /app
COPY --from=builder /app/target/release/zenith4k-serve /usr/local/bin/zenith4k-serve
RUN mkdir -p /app/audit && chown -R zenith:zenith /app

USER zenith
ENV ZENITH_BIND=0.0.0.0:8787 \
    ZENITH_AUDIT_DIR=/app/audit \
    RUST_LOG=zenith4k_serve=info,tower_http=info

EXPOSE 8787
ENTRYPOINT ["/usr/local/bin/zenith4k-serve"]
