# Used for local testing on Debian image

from alpine:3.9
ARG DEBIAN_FRONTEND=noninteractive

RUN apk update && \
    apk add wget curl build-base make glib glib-dev elixir openssl libwebp expat pkgconfig libressl-dev clang-libs llvm llvm-dev expat-dev && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV PATH /root/.cargo/bin:$PATH
ENV RUSTFLAGS "-C target-feature=-crt-static"

# RUN wget https://github.com/libvips/libvips/releases/download/v8.10.6/vips-8.10.6.tar.gz
# RUN tar -xf vips-8.10.6.tar.gz
# RUN cd vips-8.10.6 && \
#   ./configure && \
#   make -j16 && \
#   make install -j16


# RUN 

WORKDIR /usr/src/app

COPY mix.exs mix.lock /usr/src/app/

RUN mix local.hex --force && \
  mix deps.get --force && \
  mix local.rebar --force && \
  mix deps.compile

COPY . .

RUN mix test
