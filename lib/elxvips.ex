defmodule ResizeOptions do
  defstruct [
    width: :auto,
    height: :auto,
    fill: :auto,
  ]
end

defmodule ImageFile do
  defstruct [
    :path,
    :resize,
    :save
  ]
end

defmodule SaveOptions do
  defstruct [
    quality: 90,
    format: :auto,
    strip: true,
    path: nil,
  ]
end

defmodule Elxvips do
  @moduledoc """
  Documentation for `Elxvips`.
  """
  use Rustler, otp_app: :elxvips, crate: "lvips"

  @doc """
  Resize image.

  ## Examples

      iex> Elxvips.resize( "/path/input.png", "/path/output.jpg" )
      :ok

  """
  defp process_image(_a), do: :erlang.nif_error(:nif_not_loaded)

  def resize( image_file, opts \\ [] ) do
    with { :ok, image_file = %ImageFile{} } <- image_file do
      { :ok, %ImageFile{ image_file |
      :resize => Kernel.struct( image_file.resize, opts )
    } }
    end
  end

  def open( path ) when is_binary( path ) do
    { :ok, %ImageFile{
      :path => path,
      :resize => %ResizeOptions{
        :width => :auto,
        :height => :auto,
        :fill => :auto,
      }
    } }
  end

  def jpg( image_file, path, opts \\ [] ) when is_binary( path ) do
    with { :ok, image_file = %ImageFile{} } <- image_file do
      image_file = %ImageFile{ image_file |
        :save => Kernel.struct( %SaveOptions{}, opts ++ [ format: :jpg, path: path ] )
      }

      process_image( image_file )
    end
  end

  def png( image_file, path, opts \\ [] ) when is_binary( path ) do
    with { :ok, image_file = %ImageFile{} } <- image_file do
      image_file = %ImageFile{ image_file |
        :save => Kernel.struct( %SaveOptions{}, opts ++ [ format: :png, path: path ] )
      }

      process_image( image_file )
    end
  end



end