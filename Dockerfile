# Used for local testing on Debian image

from ubuntu:latest
ARG DEBIAN_FRONTEND=noninteractive

RUN apt update && \
    apt install -y wget curl build-essential llvm clang libclang-dev libvips libvips-dev && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    wget https://packages.erlang-solutions.com/erlang-solutions_2.0_all.deb && \
    dpkg -i erlang-solutions_2.0_all.deb && \
    apt update && \
    apt install -y esl-erlang elixir

ENV PATH /root/.cargo/bin:$PATH

WORKDIR /usr/src/app

COPY mix.exs mix.lock /usr/src/app/

RUN mix local.hex --force && \
  mix deps.get --force && \
  mix local.rebar --force && \
  mix deps.compile

COPY . .

RUN mix test
