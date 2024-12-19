FROM rust:alpine AS build

WORKDIR /

ARG TARGETARCH \
  EVENT_NAME

RUN apk add \
  curl \
  protoc \
  musl-dev \
  gzip \
  perl \
  build-base

RUN [ "${TARGETARCH}" = "arm64" ] && ARCH=aarch64 || ARCH=x86_64 \
  && TARGET=${ARCH}-unknown-linux-musl \
  && if [ "${EVENT_NAME}" = "schedule" ]; then \
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
