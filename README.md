# WSE Trader

## Build references

### Use makefile

#### Prerequisites

In order to use makefile make sure you have installed the cargo-make crate.

```console
cargo install --no-default-features --force cargo-make
```

or

```console
cargo install --force cargo-make
```

#### Build release for linux

To build a release, execute appropriate makefile in the backend or fronend folder.

#### Frontend 

```console
cargo make --makefile makefile.toml linux-release-flow
```

This will create release frontend file in frontend_server folder.

#### Backend 

```console
cargo make --makefile makefile.toml linux-release-flow
```

This will create release backend file with dependencies in backend_server folder.

### Use Dockerfile - build image

#### Build docker image

In the docker folder execute:

#### Frontend example

```console
docker build . -t wse_frontend
```

#### Build docker image

In the docker folder execute:

#### Backend example

```console
docker build . -t wse_backend
```
### Run image

#### Run in debug mode 

When we add in the Docker file a 

```console
CMD ["bash"]
```

Then we can run the container in interactive terminal mode

```console
docker run -it wse_frontend bash
```

### Run server with port mapping of the host without the docker compose

### Backend

```console
docker run --name=be wse_backend backend --oa=0.0.0.0 --op=80
```

Then using command below we can obtain the local ip address which usually will be 172.17.0.2

```console
docker inspect be
```

### Frontend

Knowing the backend container address we can use this for running the frontend.
Also port mapping may be used so that the server is visible from outside the host.
Here we map application port 80 which is for the container. Then the port is mapped to 8080 of the host.
So now we can reach the server using host IP address and port (example 192.168.1.202:8080/index.html).

Also it is important to remember, that if the host is a virtual machine we need to configure
port forwarding between VM and our PC so here this would be 8080 and 8080 if we decide to use
same port on the PC. Also if we want to use the ip address of the PC to access our server,
we need to use the NAT connection. 

```console
docker run --name=fe -p 8080:80 wse_frontend frontend --oa=0.0.0.0 --op=80 --ra=172.17.0.2 --rp=80
```

So now our server is accessible within the VM through 172.17.0.2:80/index.html.
And from the outside of the VM though for example 192.168.1.202:8080/index.html of course if our
PC address is indeed 192.168.1.202.

## Testing

To run integration tests run the following:

### Third party mock

```console
cargo mock-test-server
```

### Backend

```console
cd backend
```

```console
cargo run -- --oa 127.0.0.1 --op 80 --companies-list-url http://127.0.0.1:8765/spolki-rating/akcje_gpw --company-indicators-url http://127.0.0.1:8765/notowania/gpw/
```

### Integration tests

```console
cargo test
```