# Docker Autoheal

[![GitHubRelease][GitHubReleaseBadge]][GitHubReleaseLink]
[![DockerPublishing][DockerPublishingBadge]][DockerLink]
[![DockerSize][DockerSizeBadge]][DockerLink]
[![DockerPulls][DockerPullsBadge]][DockerLink]

A cross-platform tool to monitor and remediate unhealthy Docker containers

Designed to be OS agnostic, flexible, and performant in large environments via concurrency and multi-threading

The `docker-autoheal` binary may be executed in a native OS or from a Docker container

## How to Use

### You must first apply `HEALTHCHECK` to your docker images

- See <https://docs.docker.com/engine/reference/builder/#healthcheck> for details

### Environment Variables

| Variable                     | Default               | Description                                           |
|:----------------------------:|:---------------------:|:-----------------------------------------------------:|
| **AUTOHEAL_CONNECTON_TYPE**  | local                    | This determines how `docker-autheal` connects to Docker (One of: local, socket, http, ssl                                                                    |
| **AUTOHEAL_CONTAINER_LABEL** | autoheal                 | This is the container label that `docker-autoheal` will use as filter criteria for monitoring - or set to `all` to simply monitor all containers on the host |
| **AUTOHEAL_STOP_TIMEOUT**    | 10                       | Docker waits `n` seconds for a container to stop before killing it during restarts <!-- (overridable via label; see below) -->                               |
| **AUTOHEAL_INTERVAL**        | 5                        | Check container health every `n` seconds              |
| **AUTOHEAL_START_DELAY**     | 0                        | Wait `n` seconds before first health check            |
| **AUTOHEAL_TCP_HOST**        | localhost                | Address of Docker host                                |
| **AUTOHEAL_TCP_PORT**        | 2375 (ssl: 2376)         | Port on which to connect to the Docker host           |
| **AUTOHEAL_TCP_TIMEOUT**     | 10                       | Time in `n` seconds before failing connection attempt |
| **AUTOHEAL_PEM_PATH**        | /opt/docker-autoheal/tls | Fully qualified path to requisite ssl certificate files (key.pem, cert.pem, ca.pem) when `AUTOHEAL_CONNECTION_TYPE=ssl`|
| **AUTOHEAL_APPRISE_URL**    |                          |URL to post messages to the apprise following actions on unhealthy container                                             |
| **AUTOHEAL_WEBHOOK_KEY**    |                          |KEY to post messages to the webhook following actions on unhealthy container                                             |
| **AUTOHEAL_WEBHOOK_URL**    |                          |URL to post messages to the webhook following actions on unhealthy container                                             |

### Binary Options

Used when executed in native OS (NOTE: The environment variables are also accepted)

```bash
Options:
    -c, --connection-type <CONNECTION_TYPE>
                        One of local, socket, http, or ssl
    -l, --container-label <CONTAINER_LABEL>
                        Container label to monitor (e.g. autoheal)
    -t, --stop-timeout <STOP_TIMEOUT>
                        Time in seconds to wait for action to complete
    -i, --interval <INTERVAL>
                        Time in seconds to check health
    -d, --start-delay <START_DELAY>
                        Time in seconds to wait for first check
    -n, --tcp-host <TCP_HOST>
                        The hostname or IP address of the Docker host (when -c
                        http or ssl)
    -p, --tcp-port <TCP_PORT>
                        The tcp port number of the Docker host (when -c http
                        or ssl)
    -k, --key-path <KEY_PATH>
                        The fully qualified path to requisite ssl PEM files
    -a, --apprise-url <APPRISE_URL>
                        The apprise url
    -j, --webhook-key <WEBHOOK_KEY>
                        The webhook json key
    -w, --webhook-url <WEBHOOK_URL>
                        The webhook url
    -h, --help          Print help
    -v, --version       Print version information
```

### Local

```bash
/usr/local/bin/docker-autoheal --container-label all > /var/log/docker-autoheal.log &
```

Will connect to the local Docker host and monitor all containers

### Socket

```bash
docker run -d \
    --name docker-autoheal \
    --restart=always \
    --read-only \
    --env="AUTOHEAL_CONNECTION_TYPE=socket" \
    --env="AUTOHEAL_CONTAINER_LABEL=autoheal" \
    --volume=/var/run/docker.sock:/var/run/docker.sock \
    ghcr.io/tmknight/docker-autoheal:latest
```

Will connect to the Docker host via unix socket location /var/run/docker.sock or Windows named pipe location //./pipe/docker_engine and monitor only containers with a label named `autoheal`

### HTTP

```bash
docker run -d \
    --name docker-autoheal \
    --restart=always \
    --read-only \
    --env="AUTOHEAL_CONNECTION_TYPE=http" \
    --env="AUTOHEAL_CONTAINER_LABEL=watch-me" \
    --env="AUTOHEAL_TCP_HOST=MYHOST" \
    --env="AUTOHEAL_TCP_PORT=2375" \
    ghcr.io/tmknight/docker-autoheal:latest
```

Will connect to the Docker host via hostname or IP and the specified port and monitor only containers with a label named `watch-me`

### Logging

```bash
2024-01-23 03:03:23-0500 [WARNING] [nordvpn] Container (886d37fd9f5c) is unhealthy with 3 failures
2024-01-23 03:03:23-0500 [WARNING] [nordvpn] Restarting container (886d37fd9f5c) with 10s timeout
2024-01-23 03:03:34-0500 [   INFO] [nordvpn] Restart of container (886d37fd9f5c) was successful
2024-01-23 03:04:48-0500 [WARNING] [privoxy] Container (74f74eb7b2d0) is unhealthy with 3 failures
2024-01-23 03:04:48-0500 [WARNING] [privoxy] Restarting container (74f74eb7b2d0) with 10s timeout
2024-01-23 03:04:59-0500 [   INFO] [privoxy] Restart of container (74f74eb7b2d0) was successful
```

Example log output when docker-autoheal is in action

## Other Info

### Docker Labels

a) Apply the label `autoheal=true` to your container to have it watched (only the label name is assessed, the value is not currently used)

b) Set ENV `AUTOHEAL_CONTAINER_LABEL` to that label name (e.g. `AUTOHEAL_CONTAINER_LABEL=autoheal`)

OR

c) Set ENV `AUTOHEAL_CONTAINER_LABEL=all` to watch all running containers

### SSL Connection Type

See <https://docs.docker.com/engine/security/https/> for how to configure TCP with mTLS

The certificates and keys need these names:

- ca.pem
- cert.pem
- key.pem

### Docker Timezone

If you need the `docker-autoheal` container timezone to match the local machine, you can map `/etc/localtime`

docker run ... -v /etc/localtime:/etc/localtime:ro

## Credits

- [willfarrell](https://github.com/willfarrell)

[GitHubReleaseBadge]: https://github.com/tmknight/docker-autoheal/actions/workflows/github-release.yml/badge.svg
[GitHubReleaseLink]: https://github.com/tmknight/docker-autoheal/releases
[DockerPublishingBadge]: https://github.com/tmknight/docker-autoheal/actions/workflows/docker-publish.yml/badge.svg
[DockerPullsBadge]: https://badgen.net/docker/pulls/tmknight88/docker-autoheal?icon=docker&label=Docker+Pulls&labelColor=black&color=green
[DockerSizeBadge]: https://badgen.net/docker/size/tmknight88/docker-autoheal?icon=docker&label=Docker+Size&labelColor=black&color=green
[DockerLink]: https://hub.docker.com/r/tmknight88/docker-autoheal
