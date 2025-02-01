FROM rust:latest AS build

WORKDIR /
USER root

ARG TARGETARCH \
  EVENT_NAME

# RUN apk update \
#   && apk add \
#   build-base \
#   curl \
#   gzip \
#   musl-dev \
#   openssl-dev \
#   perl \
#  protoc

RUN apt-get update && apt-get install -y \
  build-essential \
  curl \
  gzip \
  musl-dev \
  libssl-dev \
  perl \
  protobuf-compiler

RUN [ "${TARGETARCH}" = "arm64" ] && ARCH=aarch64 || ARCH=x86_64 \
  && TARGET=${ARCH}-unknown-linux-musl \
  && rustup target add ${TARGET} \
  && if [ "${EVENT_NAME}" = "schedule" ] || [ "${EVENT_NAME}" = "workflow_dispatch" ]; then \
    cargo install --git https://github.com/tmknight/docker-autoheal --branch main --target ${TARGET} && \
    mv /usr/local/cargo/bin/docker-autoheal /; \
  else \
    curl -sLO https://github.com/tmknight/docker-autoheal/releases/latest/download/docker-autoheal-${TARGET}.tar.gz && \
    tar -xvf docker-autoheal-${TARGET}.tar.gz; \
  fi \
  && chmod +x docker-autoheal

FROM alpine:latest

COPY --from=build /docker-autoheal /docker-autoheal

RUN apk update \
  && apk upgrade --no-cache --no-progress --purge \
  && rm -rf \
  /tmp/* \
  /var/tmp/*

HEALTHCHECK --interval=5s \
  CMD pgrep -f docker-autoheal || exit 1

ENTRYPOINT ["/docker-autoheal"]
