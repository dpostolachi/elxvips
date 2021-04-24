# Used for local testing on Debian image

from alpine:3.9
ARG DEBIAN_FRONTEND=noninteractive

RUN apk update && \
    apk add wget curl build-base make glib glib-dev elixir openssl libwebp expat pkgconfig libressl-dev clang-libs llvm llvm-dev expat-dev && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV PATH /root/.cargo/bin:$PATH

WORKDIR /usr/src/app

COPY mix.exs mix.lock /usr/src/app/

RUN mix local.hex --force && \
  mix deps.get --force && \
  mix local.rebar --force && \
  mix deps.compile

COPY . .

RUN mix test
