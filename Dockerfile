### BASE

FROM ubuntu:22.04 as base
WORKDIR /app
RUN apt update

# rust
RUN apt install curl -y
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# bevy
RUN apt install -y g++ pkg-config libx11-dev libasound2-dev libudev-dev

# wasm deps
RUN rustup target install wasm32-unknown-unknown
RUN cargo install wasm-server-runner

# linux-only optimizaiton: https://bevyengine.org/learn/book/getting-started/setup/
RUN apt install mold clang  -y

### DEV

FROM base AS dev
COPY . .