# mc-status-rs

This is a minimal discord bot to service an SMP that I play with friends. 
It lets us check the server status in Discord to see how many players are on, 
what the ping is, what the IP is, etc.

The program is written in Rust and Dockerized via Github Actions to run on my Kubernetes cluster and deployed with ArgoCD.

Three values need to be provided to the environment:

```shell
DISCORD_TOKEN=<Discord token>
MC_SERVER_IP=<Minecraft server IP>
MAPS_ADDRESS=<Address to bluemaps webserver> # Optional
```

Afterwards, the bot can be run inside of the docker image:

```shell
docker run -d ghcr.io/vaughnw128/mc-status-rs:<release-number>
```