# Install, test, and run the project through MagiCore inside Docker.
# Mọi thao tác install, test và chạy project trong Docker đều đi qua MagiCore.
ARG MGC_IMAGE=ghcr.io/mingd-153/magicore:1.1.0-rc.10
FROM ${MGC_IMAGE} AS mgc-cli
FROM node:26.12.0-trixie-slim

WORKDIR /app
COPY --from=mgc-cli /usr/local/bin/mgc /usr/local/bin/mgc
COPY --from=mgc-cli /usr/lib/libz.so.1 /usr/lib/libz.so.1
COPY --from=mgc-cli /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=mgc-cli --chown=65532:65532 /mgc-data /mgc-data
COPY mgc-container-entrypoint /usr/local/bin/mgc-container-entrypoint
ENV MGC_DATA_DIR=/mgc-data
ENV HOME=/tmp
ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
COPY . .
RUN chown -R 65532:65532 /app /mgc-data
RUN chmod 0555 /usr/local/bin/mgc-container-entrypoint
USER 65532:65532

RUN mgc install-web --frozen
RUN mgc test

EXPOSE 4315
ENTRYPOINT ["/usr/local/bin/mgc-container-entrypoint"]
CMD ["run", "start"]
