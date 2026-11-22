FROM cgr.dev/chainguard/glibc-dynamic:latest

ARG TARGETARCH
ARG BINARY_NAME=mc-status-rs

COPY dist/linux/${TARGETARCH}/${BINARY_NAME} /usr/local/bin/app

ENTRYPOINT ["/usr/local/bin/app"]
