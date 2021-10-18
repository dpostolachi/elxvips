# Elxvips
[![Build Status](https://travis-ci.org/dpostolachi/elxvips.png?branch=master)](https://travis-ci.org/dpostolachi/elxvips)

Experimental Elixir bindings to libvips for image processing, since it's supposed to be [faster](https://github.com/libvips/libvips/wiki/Speed-and-memory-use) than GraphicsMagick/ImageMagick. It is based on the existing Rust [bindings](https://github.com/augustocdias/libvips-rust-bindings) to libVips. To make it work you will require libVips and the rust compiler. Please refer to [Dockerfile](https://github.com/dpostolachi/elxvips/blob/master/Dockerfile) for dependencies.

## Installation

If [available in Hex](https://hex.pm/packages/elxvips), the package can be installed
by adding `elxvips` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:elxvips, "~> 0.1.1"}
  ]
end
```

## Example

```elixir
import Elxvips

from_file( "image.png" )
|> resize( height: 100 )
|> jpg( strip: true )
|> to_file( "output.jpg" )
{ :ok, %ImageFile{} }
```

Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc)
and published on [HexDocs](https://hexdocs.pm). Once published, the docs can
be found at [https://hexdocs.pm/elxvips](https://hexdocs.pm/elxvips).

