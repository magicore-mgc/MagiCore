# MagiCore CLI — multi-stage Docker build
# Usage: docker build -t magicore .
#        docker run --rm magicore --help

ARG RUST_IMAGE=docker.io/library/rust:1.99.0-slim-trixie
ARG GO_IMAGE=docker.io/library/golang:1.27.2-trixie
ARG RUNTIME_IMAGE=gcr.io/distroless/cc-debian13

FROM ${GO_IMAGE} AS go-toolchain

# ---- Build stage ----
FROM ${RUST_IMAGE} AS build
WORKDIR /build
COPY --from=go-toolchain /usr/local/go /usr/local/go
ENV PATH="/usr/local/go/bin:${PATH}"

RUN apt-get update -qq && apt-get install -y -qq \
    build-essential pkg-config libssl-dev zlib1g-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY core/ core/
COPY adapters/ adapters/
COPY assets/ assets/
COPY cli/ cli/
COPY tools/ tools/
COPY tests/e2e/ tests/e2e/

RUN cargo build --release --package mgc --locked && \
    install -Dm755 target/release/mgc /mgc

# distroless/cc-debian12 does not guarantee zlib is present, while libz-sys
# links the CLI against libz.so.1. Resolve the multi-arch path in the builder
# and copy the real library so the final image needs no shell or external link.
RUN set -eu; \
    mkdir -p /runtime-libs; \
    mkdir -p /mgc-data; \
    for lib in /lib/*-linux-gnu/libz.so.1 /usr/lib/*-linux-gnu/libz.so.1; do \
      if [ -e "$lib" ]; then cp -L "$lib" /runtime-libs/libz.so.1; break; fi; \
    done; \
    test -s /runtime-libs/libz.so.1

# ---- Runtime stage ----
FROM ${RUNTIME_IMAGE}
LABEL org.opencontainers.image.source="https://github.com/mingd-153/MagiCore"
COPY --from=build /runtime-libs/libz.so.1 /usr/lib/libz.so.1
COPY --from=build /mgc /usr/local/bin/mgc
COPY --from=build --chown=65532:65532 /mgc-data /mgc-data
ENV MGC_DATA_DIR=/mgc-data
ENV HOME=/tmp
USER 65532:65532
ENTRYPOINT ["/usr/local/bin/mgc"]
CMD ["--help"]
