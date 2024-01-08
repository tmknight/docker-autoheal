# Docker Autoheal

Monitor for and remediation of unhealthy Docker containers

The `docker-autoheal` binary may be executed via a native OS or via a Docker container

## ENV Defaults

| Variable                     | Default               | Description                                                                                                                                        |
|:----------------------------:|:---------------------:|:--------------------------------------------------------------------------------------------------------------------------------------------------:|
| **AUTOHEAL_CONNECTON_TYPE**  | local                 | This determines how `docker-autheal` connects to Docker (One of: local, socket, http                                                               |
| **AUTOHEAL_CONTAINER_LABEL** | autoheal              | This is the label (set to `true`) that `docker-autoheal` will monitor and remediate - or set to `all` to simply monitor all containers on the host |
| **AUTOHEAL_STOP_TIMEOUT**    | 10                    | Docker waits `n` seconds for a container to stop before killing it during restarts <!-- (overridable via label; see below) -->                     |
| **AUTOHEAL_INTERVAL**        | 5                     | Check container health every`n` seconds**                                                                                                          |
| **AUTOHEAL_START_DELAY**     | 0                     | Wait `n` seconds before first health check                                                                                                         |
| **AUTOHEAL_TCP_HOST**        | localhost             | Address of Docker host                                                                                                                             |
| **AUTOHEAL_TCP_PORT**        | 2375                  | Port on which to connect to the Docker host                                                                                                        |
| **AUTOHEAL_TCP_TIMEOUT**     | 10                    | Time in `n` seconds before failing connection attempt                                                                                              |
|
<!-- | **AUTOHEAL_KEY_PATH** | /opt/docker-autoheal/tls/key.pem                                                                                                                   | Fully qualified path to key.pem |
<!-- | **AUTOHEAL_KEY_PATH**        | /opt/docker-autoheal/tls/key.pem  | Fully qualified path to key.pem                                                                                                                    |
| **AUTOHEAL_CERT_PATH**       | /opt/docker-autoheal/tls/cert.pem | Fully qualified path to cert.pem                                                                                                                   |
| **AUTOHEAL_CA_PATH**         | /opt/docker-autoheal/tls/ca.pem   | Fully qualified path to ca.pem                                                                                                                     | -->
<!-- |WEBHOOK_URL                      |            |Post messages to the webhook following actions on unhealthy container                                                          | -->

<!--
### Optional Container Labels

| Label                             | Value    | Description                                                                                                                       |
|:---------------------------------:|:--------:|:---------------------------------------------------------------------------------------------------------------------------------:|
| **autoheal.stop.timeout**         | 20       | Per container override of the stop timeout (in seconds) during restart                                                            |
-->

## How to use

### You must first apply `HEALTHCHECK` to your docker images

- See <https://docs.docker.com/engine/reference/builder/#healthcheck> for details

### Local

```bash
export AUTOHEAL_CONTAINER_LABEL=all
/usr/local/bin/docker-autoheal > /var/log/docker-autoheal.log &
```

### Socket

```bash
docker run -d \
    --name autoheal \
    --restart=always \
    -e AUTOHEAL_CONTAINER_LABEL=all \
    -v /var/run/docker.sock:/var/run/docker.sock \
    tmknight/docker-autoheal
```

### Http

```bash
docker run -d \
    --name autoheal \
    --restart=always \
    -e AUTOHEAL_CONTAINER_LABEL=all \
    -e DOCKER_SOCK=tcp://HOST:PORT \
    -v /path/to/certs/:/certs/:ro \
    tmknight/docker-autoheal
```

## Other info

a) Apply the label `autoheal=true` to your container to have it watched

b) Set ENV `AUTOHEAL_CONTAINER_LABEL=all` to watch all running containers

OR

c) Set ENV `AUTOHEAL_CONTAINER_LABEL` to existing label name that has the value `true`

<!--
See <https://docs.docker.com/engine/security/https/> for how to configure TCP with mTLS

The certificates and keys need these names:

- ca.pem
- client-cert.pem
- client-key.pem
-->

### Change Timezone

If you need the `docker-autoheal` container timezone to match the local machine, you can map `/etc/localtime`

docker run ... -v /etc/localtime:/etc/localtime:ro

<!--
## Testing

```bash
docker build -t autoheal .

docker run -d \
    -e AUTOHEAL_CONTAINER_LABEL=all \
    -v /var/run/docker.sock:/var/run/docker.sock \
    autoheal
```
-->

## Credits

- [willfarrell](https://github.com/willfarrell)
