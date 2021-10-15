# Used for local testing on Debian image

FROM alpine:3.13
RUN apk --no-cache add \
  elixir \
  vips \
  vips-dev \
  rust \
  cargo \
  clang \
  build-base \
  git \
  libc6-compat \
  libwebp-dev

ENV PATH /root/.cargo/bin:$PATH

WORKDIR /usr/src/app

COPY mix.exs mix.lock /usr/src/app/

RUN mix local.hex --force && \
  mix deps.get --force && \
  mix local.rebar --force && \
  mix deps.compile

COPY . .

RUN mix test
