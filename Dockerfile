# MagiCore CLI — multi-stage Docker build
# Usage: docker build -t magicore .
#        docker run --rm magicore --help

ARG RUST_IMAGE=docker.io/library/rust:1.94-slim-bookworm
ARG GO_IMAGE=docker.io/library/golang:1.27.1-bookworm
ARG RUNTIME_IMAGE=gcr.io/distroless/cc-debian12

FROM ${GO_IMAGE} AS go-toolchain

# ---- Build stage ----
FROM ${RUST_IMAGE} AS build
WORKDIR /build
COPY --from=go-toolchain /usr/local/go /usr/local/go
ENV PATH="/usr/local/go/bin:${PATH}"

RUN apt-get update -qq && apt-get install -y -qq \
    build-essential pkg-config zlib1g-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY core/ core/
COPY adapters/ adapters/
COPY cli/ cli/
COPY tools/ tools/
COPY tests/e2e/ tests/e2e/

RUN cargo build --release --package mgc --locked && \
    install -Dm755 target/release/mgc /mgc

# ---- Runtime stage ----
FROM ${RUNTIME_IMAGE}
COPY --from=build /mgc /usr/local/bin/mgc
ENTRYPOINT ["/usr/local/bin/mgc"]
CMD ["--help"]
