defmodule ResizeOptions do
  defstruct [
    width: :auto,
    height: :auto,
    fill: :auto,
  ]
end

defmodule ImageFile do
  defstruct [
    path: :nil,
    bytes: [],
    resize: :nil,
    save: nil
  ]
end

defmodule ImageBytes do
  defstruct [
    bytes: [],
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

  defp vips_process_image(_a), do: :erlang.nif_error(:nif_not_loaded)
  defp vips_set_concurrency(_a), do: :erlang.nif_error(:nif_not_loaded)
  defp vips_get_image_sizes(_a), do: :erlang.nif_error(:nif_not_loaded)
  defp vips_to_bytes(_a), do: :erlang.nif_error(:nif_not_loaded)

  def resize( image_file, opts \\ [] ) do
    with { :ok, image_file = %ImageFile{} } <- image_file do
      { :ok, %ImageFile{ image_file |
      :resize => Kernel.struct( image_file.resize, opts )
    } }
    end
  end

  @doc """
  Will create an ImageFile struct from path. This struct will be used for further processing.

  ## Examples

      iex> Elxvips.open( "/path/input.png" )
      %ImageFile{}

  """
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

  @doc """
  Will save the ImageFile in jpeg format to a specified path. Accepts quality and strip options.
  By default quality is set to 90.

  ## Examples

      iex> Elxvips.open( "/path/input.png" )
      iex> |> Elxvips.jpg( "/path/output.jpg", strip: true, quality: 72 ) 
      { :ok, %ImageFile{ :path => "/path/output.jpg", ... } }

  """
  def jpg( image_file, path, opts \\ [] ) when is_binary( path ) do
    with { :ok, image_file = %ImageFile{} } <- image_file do
      image_file = %ImageFile{ image_file |
        :save => Kernel.struct( %SaveOptions{}, opts ++ [ format: :jpg, path: path ] )
      }

      with :ok <- vips_process_image( image_file ) do
        { :ok, %ImageFile{
          :path => path,
          :resize => %ResizeOptions{
            :width => :auto,
            :height => :auto,
            :fill => :auto,
          }
        } }
      else
        err -> err
      end
    end
  end

  @doc """
  Will save the ImageFile in png format to a specified path. Accepts quality and strip options.
  By default quality is set to 90.

  ## Examples

      iex> Elxvips.open( "/path/input.png" )
      iex> |> Elxvips.png( "/path/output.png", strip: true, quality: 72 ) 
      { :ok, %ImageFile{ :path => "/path/output.png", ... } }
  """
  def png( image_file, path, opts \\ [] ) when is_binary( path ) do
    with { :ok, image_file = %ImageFile{} } <- image_file do
      image_file = %ImageFile{ image_file |
        :save => Kernel.struct( %SaveOptions{}, opts ++ [ format: :png, path: path ] )
      }

      with :ok <- vips_process_image( image_file ) do
        { :ok, %ImageFile{
          :path => path,
          :resize => %ResizeOptions{
            :width => :auto,
            :height => :auto,
            :fill => :auto,
          }
        } }
      else
        err -> err
      end
    end
  end

  @doc """
  Will set the concurrency of libVips to a specified number.
  By default it will check for "VIPS_CONCURRENCY" environment variable, if none if found it will fallback to number of cpu cores.

  ## Examples

      iex> Elxvips.set_concurrency( 4 )
      :ok
  """
  def set_concurrency( concurrency ) when is_integer( concurrency ) do
    vips_set_concurrency( concurrency )
  end

  @doc """
  Will set the concurrency of libVips to a specified number.
  By default it will check for "VIPS_CONCURRENCY" environment variable, if none if found it will fallback to number of cpu cores.

  ## Examples

      iex> Elxvips.set_concurrency( 4 )
      :ok
  """
  def get_image_sizes( path ) when is_binary( path ) do
    vips_get_image_sizes( path )
  end

  def get_image_sizes( image_file = %ImageFile{} ) do
    vips_get_image_sizes( image_file.path )
  end
  def get_image_sizes( {:ok, image_file = %ImageFile{} } ) do
    vips_get_image_sizes( image_file.path )
  end

  def to_bytes( path ) when is_binary( path ) do
    vips_to_bytes( path )
  end

  def to_bytes( image_file = %ImageFile{ :path => path } ) when is_binary( path ) do
  end

  def from_bytes( { :ok, bytes } ) when is_list( bytes ) do
    { :ok, %ImageBytes{
      :bytes => bytes,
    } }
  end

  # def to_bytes()

end