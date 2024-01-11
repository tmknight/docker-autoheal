FROM cgr.dev/chainguard/wolfi-base:latest

ARG BIN_VER

COPY ./bin/docker-autoheal-musl_${BIN_VER} /docker-autoheal

HEALTHCHECK --interval=5s \
    CMD pgrep -f docker-autoheal || exit 1

ENTRYPOINT ["/docker-autoheal"]
