### BASE

FROM ubuntu:22.04 as base
WORKDIR /app
RUN apt update

# rust
RUN apt install curl
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# bevy
RUN apt install -y g++ pkg-config libx11-dev libasound2-dev libudev-dev


### DEV

FROM base AS dev
COPY . .

# bevy optimizations

# linux-only: https://bevyengine.org/learn/book/getting-started/setup/
RUN apt install mold clang 
