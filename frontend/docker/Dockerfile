FROM ubuntu

RUN rm /bin/sh && ln -s /bin/bash /bin/sh
RUN apt -y update
RUN apt -y install curl
RUN apt-get update -y
RUN apt-get install -y pkg-config
RUN apt-get install -y libssl-dev
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN apt install build-essential -y
RUN apt-get install git -y
RUN git clone https://github.com/meeetju/wse_trader.git
RUN cargo install cargo-make
RUN cd wse_trader/frontend && cargo make --makefile makefile.toml linux-release-flow
ENV PATH="/wse_trader/frontend/frontend_server:${PATH}"

ENTRYPOINT frontend