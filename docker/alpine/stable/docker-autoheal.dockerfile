FROM alpine:latest as build

WORKDIR /

ARG TARGETARCH

RUN apk add \
  curl \
  protoc \
  musl-dev \
  gzip

RUN [ "${TARGETARCH}" == "amd64" ] && ARCH=x86_64 || ARCH=aarch64 \
  && curl -sLO https://github.com/tmknight/docker-autoheal/releases/latest/download/docker-autoheal-${ARCH}-unknown-linux-musl.tar.gz \
  && tar -xvf docker-autoheal-${ARCH}-unknown-linux-musl.tar.gz \
  && chmod +x docker-autoheal

FROM alpine:latest

COPY --from=build /docker-autoheal /docker-autoheal

HEALTHCHECK --interval=5s \
    CMD pgrep -f docker-autoheal || exit 1

ENTRYPOINT ["/docker-autoheal"]
