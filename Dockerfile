# Used for local testing on Debian image

from alpine:latest
ARG DEBIAN_FRONTEND=noninteractive

RUN apk update && \
    apk add wget curl build-base glib elixir openssl libwebp pkgconfig libressl-dev glib-dev clang-libs llvm llvm-dev && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV PATH /root/.cargo/bin:$PATH
ENV RUSTFLAGS "-C target-feature=-crt-static"

WORKDIR /usr/src/app

COPY mix.exs mix.lock /usr/src/app/

RUN mix local.hex --force && \
  mix deps.get --force && \
  mix local.rebar --force && \
  mix deps.compile

COPY . .

RUN mix test
