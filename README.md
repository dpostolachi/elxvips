# Elxvips
[![Build Status](https://travis-ci.org/dpostolachi/elxvips.png?branch=master)](https://travis-ci.org/dpostolachi/elxvips)

Experimental Elixir bindings to libvips for image processing, since it's supposed to be [faster](https://github.com/libvips/libvips/wiki/Speed-and-memory-use) than GraphicsMagick/ImageMagick. It is based on the existing Rust [bindings](https://github.com/augustocdias/libvips-rust-bindings) to libVips. To make it work you will require libVips. Please refer to [Dockerfile](https://github.com/dpostolachi/elxvips/blob/master/Dockerfile) for dependencies.

## Documentation

Full documentation can be found at [https://hexdocs.pm/elxvips](https://hexdocs.pm/elxvips/Elxvips.html).

## Example

```elixir
import Elxvips

from_file( "image.png" )
|> resize( height: 100 )
|> jpg( strip: true )
|> to_file( "output.jpg" )
{ :ok, %ImageFile{} }
```

## Installation

The [package](https://hex.pm/packages/elxvips) can be installed by adding `elxvips` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:elxvips, "~> 0.1.7"}
  ]
end
```


## Precompiled NIFs

Since version `0.1.3` Elxvips comes with some precompiled nifs, and depends only on libvips.

### Installing libvips

#### MacOS
```bash
brew install vips llvm
```

#### Ubuntu/Debian
```bash
apt install libvips
```

### Install build dependencies

In case you require to compile the nifs from source, you would need to install rust and pkg-config.

#### MacOS
```bash
# install rust if not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Ubuntu/Debian
```bash
# install rust if not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

apt install rustc clang
```

