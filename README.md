# Docker-Autoheal

[![GitHubRelease][GitHubReleaseBadge]][GitHubReleaseLink]
[![GitHubAssetDl][GitHubAssetDlBadge]][GitHubAssetDlLink]
[![DockerPublishing][DockerPublishingBadge]][DockerLink]
[![DockerSize][DockerSizeBadge]][DockerLink]
[![DockerPulls][DockerPullsBadge]][DockerLink]

A cross-platform tool to monitor and remediate unhealthy Docker containers

Written in Rust and designed to be OS agnostic, flexible, and performant in large environments via concurrency and multi-threading

The `docker-autoheal` binary may be executed in a native OS or from a Docker container

## How to Use

### You must first apply `HEALTHCHECK` to your docker images

- See <https://docs.docker.com/engine/reference/builder/#healthcheck> for details

### Environment Variables

| Variable                     | Default                  | Description                                           |
|:----------------------------:|:------------------------:|:-----------------------------------------------------:|
| **AUTOHEAL_CONNECTION_TYPE** | local                    | This determines how `docker-autoheal` connects to Docker (One of: local, socket, http, ssl                               |
| **AUTOHEAL_STOP_TIMEOUT**    | 10                       | Docker waits `n` seconds for a container to stop before killing it during restarts (override via label; see below)       |
| **AUTOHEAL_INTERVAL**        | 5                        | Check container health every `n` seconds              |
| **AUTOHEAL_START_DELAY**     | 0                        | Wait `n` seconds before first health check            |
| **AUTOHEAL_POST_ACTION**     |                          | The absolute path of an executable to be run after restart attempts; container `name`, `id` and `stop-timeout` are passed as arguments in that order                                             |
| **AUTOHEAL_MONITOR_ALL**     | FALSE                    | Set to `TRUE` to simply monitor all containers on the host or leave as `FALSE` and control via `autoheal.monitor.enable` |
| **AUTOHEAL_LOG_ALL**         | FALSE                    | Allow (`TRUE`/`FALSE`) logging (and webhook/apprise if set) for containers with `autostart.restart.enable=FALSE`         |
| **AUTOHEAL_TCP_HOST**        | localhost                | Address of Docker host                                |
| **AUTOHEAL_TCP_PORT**        | 2375 (ssl: 2376)         | Port on which to connect to the Docker host           |
| **AUTOHEAL_TCP_TIMEOUT**     | 10                       | Time in `n` seconds before failing connection attempt |
| **AUTOHEAL_PEM_PATH**        | /opt/docker-autoheal/tls | Absolute path to requisite ssl certificate files (key.pem, cert.pem, ca.pem) when `AUTOHEAL_CONNECTION_TYPE=ssl`         |
| **AUTOHEAL_APPRISE_URL**     |                          | URL to post messages to the apprise following actions on unhealthy container                                             |
| **AUTOHEAL_WEBHOOK_KEY**     |                          | KEY to post messages to the webhook following actions on unhealthy container                                             |
| **AUTOHEAL_WEBHOOK_URL**     |                          | URL to post messages to the webhook following actions on unhealthy container                                             |

### Optional Container Labels

| Label                        | Default | Description                                                                                                                                 |
|:----------------------------:|:-------:|:-------------------------------------------------------------------------------------------------------------------------------------------:|
| **autoheal.stop.timeout**    |         | Per container override (in seconds) of `AUTOHEAL_STOP_TIMEOUT` during restart (e.g. Some container routinely takes longer to cleanly exit)  |
| **autoheal.monitor.enable**  | FALSE   | Per container override (true/false) to control if should be monitored (e.g. If you have a large number of containers that you wish to monitor and restart, apply this label as `FALSE` to the few that you do not wish to monitor and set `AUTOHEAL_MONITOR_ALL` to `TRUE`)                                                                                  |
| **autoheal.restart.enable**  | TRUE    | Per container override (true/false) to control if should restart on unhealthy (e.g. If you have a large number of containers that you wish to monitor and restart, apply this label as `FALSE` to the few that you do not wish to restart and set `AUTOHEAL_MONITOR_ALL` to `TRUE`)                                                                       |

### Binary Options

Used when executed in native OS (NOTE: The environment variables are also accepted)

```bash
Options:
    -a, --apprise-url <APPRISE_URL>
                        The apprise url
    -c, --connection-type <CONNECTION_TYPE>
                        One of local, socket, http, or ssl
    -d, --start-delay <START_DELAY>
                        Time in seconds to wait for first check
    -h, --help          Print help
    -i, --interval <INTERVAL>
                        Time in seconds to check health
    -j, --webhook-key <WEBHOOK_KEY>
                        The webhook json key string
    -k, --key-path <KEY_PATH>
                        The absolute path to requisite ssl PEM files
    -l, --log-all       Enable logging of unhealthy containers where restart
                        is disabled (WARNING, this could be chatty)
    -m, --monitor-all   Enable monitoring off all containers that have a
                        healthcheck
    -n, --tcp-host <TCP_HOST>
                        The hostname or IP address of the Docker host (when -c
                        http or ssl)
    -p, --tcp-port <TCP_PORT>
                        The tcp port number of the Docker host (when -c http
                        or ssl)
    -s, --stop-timeout <STOP_TIMEOUT>
                        Time in seconds to wait for action to complete
    -t, --tcp-timeout <TCP_TIMEOUT>
                        Time in seconds to wait for connection to complete
    -w, --webhook-url <WEBHOOK_URL>
                        The webhook url
    -P, --post-action <SCRIPT_PATH>
                        The absolute path to a script that should be executed
                        after container restart
    -V, --version       Print version information
```

### Local

```bash
/usr/local/bin/docker-autoheal --monitor-all > /var/log/docker-autoheal.log &
```

Will connect to the local Docker host and monitor all containers

### Socket

```bash
docker run -d --read-only \
    --user=[uid]:[gid]
    --name docker-autoheal \
    --network=none \
    --restart=always \
    --env="AUTOHEAL_CONNECTION_TYPE=socket" \
    --env="AUTOHEAL_MONITOR_ALL=true" \
    --volume=/var/run/docker.sock:/var/run/docker.sock:ro \
    tmknight88/docker-autoheal:latest
```

Will connect to the Docker host via unix socket location /var/run/docker.sock or Windows named pipe location //./pipe/docker_engine and monitor all containers as the user with the specified `uid:gid`

### HTTP

```bash
docker run -d --read-only \
    --user=[uid]:[gid]
    --name docker-autoheal \
    --restart=always \
    --env="AUTOHEAL_CONNECTION_TYPE=http" \
    --env="AUTOHEAL_TCP_HOST=MYHOST" \
    --env="AUTOHEAL_TCP_PORT=2375" \
    tmknight88/docker-autoheal:latest
```

Will connect to the Docker host via hostname or IP and the specified port and monitor only containers with a label `autoheal.monitor.enable=true` as the user with the specified `uid:gid`

### Logging

```bash
2024-01-23 03:03:23-0500 [WARNING] [nordvpn] Container (886d37fd9f5c) is unhealthy with 3 failures
2024-01-23 03:03:23-0500 [WARNING] [nordvpn] Container (886d37fd9f5c) last output: [4] Status: Unstable
2024-01-23 03:03:23-0500 [WARNING] [nordvpn] Restarting container (886d37fd9f5c) with 10s timeout
2024-01-23 03:03:34-0500 [   INFO] [nordvpn] Restart of container (886d37fd9f5c) was successful
2024-01-23 03:04:48-0500 [WARNING] [privoxy] Container (74f74eb7b2d0) is unhealthy with 3 failures
2024-01-23 03:04:48-0500 [WARNING] [privoxy] Container (74f74eb7b2d0) last output: [-1] Health check exceeded timeout (3s)
2024-01-23 03:04:48-0500 [WARNING] [privoxy] Restarting container (74f74eb7b2d0) with 10s timeout
2024-01-23 03:04:59-0500 [   INFO] [privoxy] Restart of container (74f74eb7b2d0) was successful
```

Example log output when docker-autoheal is in action

## Other Info

### Docker Labels

a) Apply the label `autoheal.monitor.enable=true` to your container to have it watched

OR

b) Set ENV `AUTOHEAL_MONITOR_ALL=true` (or apply `--monitor-all` to the binary) to watch all running containers

### SSL Connection Type

See <https://docs.docker.com/engine/security/https/> for how to configure TCP with mTLS

The certificates and keys need these names:

- ca.pem
- cert.pem
- key.pem

### Docker Security

Additional security can be obtained by:

- Use a unique user for monitoring and remediating
  - Create a new user
  - Add that user to the `docker` group
  - Execute the binary or docker container with that uid:gid
- Run docker in [rootless mode](https://docs.docker.com/engine/security/rootless/)

### Docker Timezone

If you need the `docker-autoheal` container timezone to match the local machine, you can map `/etc/localtime`

```bash
docker run ... -v /etc/localtime:/etc/localtime:ro
```

### Webhook/Apprise

- The payload includes the following separated by `|`: Docker system hostname, the last health output, and the result of restart action


### A Word of Caution about Excluding from Restart and Logging Exclusions

- Excluding a container from restarts and enabling logging for excluded containers will generate numerous log messages whenever that container becomes unhealthy
- Additionally, if a webhook or apprise is also configured for those containers, they will be executed at each monitoring interval

## Credits

- [willfarrell](https://github.com/willfarrell)

[GitHubReleaseBadge]: https://img.shields.io/github/actions/workflow/status/tmknight/docker-autoheal/github-release.yml?branch=main&style=flat&logo=github&color=32c855&label=generate%20release&cacheSeconds=21600
[GitHubReleaseLink]: https://github.com/tmknight/docker-autoheal/releases
[DockerPublishingBadge]: https://img.shields.io/github/actions/workflow/status/tmknight/docker-autoheal/docker-publish.yml?branch=main&style=flat&logo=github&color=32c855&label=publish%20image&cacheSeconds=21600
[DockerPullsBadge]: https://img.shields.io/docker/pulls/tmknight88/docker-autoheal?style=flat&logo=docker&color=blue&cacheSeconds=21600
[DockerSizeBadge]: https://img.shields.io/docker/image-size/tmknight88/docker-autoheal?sort=date&arch=amd64&style=flat&logo=docker&color=blue&cacheSeconds=21600
[DockerLink]: https://hub.docker.com/r/tmknight88/docker-autoheal
[GithubAssetDlBadge]: https://img.shields.io/github/downloads/tmknight/docker-autoheal/total?style=flat&logo=github&color=32c855&label=release%20downloads&cacheSeconds=21600
[GithubAssetDlLink]: https://github.com/tmknight/docker-autoheal/releases
