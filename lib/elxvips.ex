defmodule Elxvips.ResizeOptions do
  defstruct [
    width: :auto,
    height: :auto,
    resize_type: :auto, # :type conflicts with rust keyword "type"
  ]
end

defmodule Elxvips.ImageFile do
  defstruct [
    path: :nil,
    bytes: [],
    resize: %Elxvips.ResizeOptions{
      :width => :auto,
      :height => :auto,
      :resize_type => :auto,
    },
    save: nil
  ]
end

defmodule Elxvips.ImageBytes do
  defstruct [
    path: nil,
    bytes: nil,
    resize: %Elxvips.ResizeOptions{
      :width => :auto,
      :height => :auto,
      :resize_type => :auto,
    },
    save: :nil,
  ]
end

defmodule Elxvips.SaveOptions do
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
  alias Elxvips.ImageFile, as: ImageFile
  alias Elxvips.ImageBytes, as: ImageBytes
  alias Elxvips.SaveOptions, as: SaveOptions

  # NIFs
  defp vips_set_concurrency(_a), do: :erlang.nif_error(:nif_not_loaded)
  defp vips_get_image_sizes(_a), do: :erlang.nif_error(:nif_not_loaded) # returns {:ok, { width, height } }
  defp vips_get_image_bytes_sizes(_a), do: :erlang.nif_error(:nif_not_loaded) # same but works with bytes
  defp vips_process_image_file(_a), do: :erlang.nif_error(:nif_not_loaded) # applies processing from %ImageFile{}
  defp vips_process_image_file_bytes(_a), do: :erlang.nif_error(:nif_not_loaded) # applies processing from %ImageBytes{} created from image path
  defp vips_process_image_bytes(_a), do: :erlang.nif_error(:nif_not_loaded) # applies processing from %ImageBytes{} created from image bytes

  # creating new jpeg image from an existing image path
  defp to_jpg( image_file = %ImageFile{}, path, opts ) when is_binary( path ) do
    image_file = %ImageFile{ image_file |
        :save => Kernel.struct( %SaveOptions{}, opts ++ [ format: :jpg, path: path ] )
    }

    with :ok <- vips_process_image_file( image_file ) do
      { :ok, %ImageFile{
        :path => path,
      } }
    else
      err -> err
    end
  end
  # In case the we have a image path as image_bytes
  defp to_jpg( image_bytes = %ImageBytes{ :path => path }, opts ) when is_binary( path )  do
    image_bytes = %ImageBytes{ image_bytes |
      :bytes => [], # pass empty bytes to rust
      :save => Kernel.struct( %SaveOptions{}, opts ++ [ format: :jpg, path: "" ] ),
    }

    with { :ok, bytes } <- vips_process_image_file_bytes( image_bytes ) do
      { :ok, %ImageBytes{
        :bytes => bytes,
      } }
    else
      err -> err
    end
  end
  # In case we have raw bytes
  defp to_jpg( image_bytes = %ImageBytes{ :bytes => bytes }, opts ) when is_list( bytes ) do
    image_bytes = %ImageBytes{ image_bytes |
      :path => "", # pass empty string to rust
      :save => Kernel.struct( %SaveOptions{}, opts ++ [ format: :jpg, path: "" ] ),
    }

    with { :ok, bytes } <- vips_process_image_bytes( image_bytes ) do
      { :ok, %ImageBytes{
        :bytes => bytes,
      } }
    else
      err -> err
    end
  end
  # new png image from image path
  defp to_png( image_file = %ImageFile{}, path, opts ) when is_binary( path ) do
    image_file = %ImageFile{ image_file |
        :save => Kernel.struct( %SaveOptions{}, opts ++ [ format: :png, path: path ] )
    }

    with :ok <- vips_process_image_file( image_file ) do
      { :ok, %ImageFile{
        :path => path,
      } }
    else
      err -> err
    end
  end
  # In case the we have a image path as image_bytes
  defp to_png( image_bytes = %ImageBytes{ :path => path }, opts ) when is_binary( path )  do
    image_bytes = %ImageBytes{ image_bytes |
      :bytes => [], # pass empty bytes to rust
      :save => Kernel.struct( %SaveOptions{}, opts ++ [ format: :png, path: "" ] ),
    }

    with { :ok, bytes } <- vips_process_image_file_bytes( image_bytes ) do
      { :ok, %ImageBytes{
        :bytes => bytes,
      } }
    else
      err -> err
    end
  end
  # In case we have raw bytes
  defp to_png( image_bytes = %ImageBytes{ :bytes => bytes }, opts ) when is_list( bytes ) do
    image_bytes = %ImageBytes{ image_bytes |
      :path => "", # pass empty string to rust
      :save => Kernel.struct( %SaveOptions{}, opts ++ [ format: :png, path: "" ] ),
    }

    with { :ok, bytes } <- vips_process_image_bytes( image_bytes ) do
      { :ok, %ImageBytes{
        :bytes => bytes,
      } }
    else
      err -> err
    end
  end

  @doc """
  Applies resize options to an %ImageFile{} or %ImageBytes{}, accepts :width, :height and :type (not implemented yet).
  If no width or height is specified dimensions are calculated from the input image.
  Empty resize( no :width and no :height) will produce an image with the dimensions as the original one.
  
  ## Note
  All resizes are applied on image creation, when jpg or jpg_bytes or other format functions are called.
  ## Examples
      iex> Elxvips.open( "test/input.png" )
      iex> |> Elxvips.resize( width: 300 )
      {:ok, %ImageBytes{}}

      iex> Elxvips.open( "test/input.png" )
      iex> |> Elxvips.resize()
      {:ok, %ImageBytes{}}
  """
  def resize( image_file, opts \\ [] )
  def resize( image_file = %ImageFile{}, opts ) do
    { :ok, %ImageFile{ image_file |
      :resize => Kernel.struct( image_file.resize, opts )
    } }
  end
  def resize( { :ok, image_file = %ImageFile{} }, opts ), do: resize( image_file, opts )
  def resize( image_bytes = %ImageBytes{}, opts ) do
    { :ok, %ImageBytes{ image_bytes |
      :resize => Kernel.struct( image_bytes.resize, opts )
    } }
  end
  def resize( { :ok, image_bytes = %ImageBytes{} }, opts ), do: resize( image_bytes, opts )

  @doc """
  Will create an ImageFile struct from path. This struct will be used for further processing.

  ## Examples

      iex> Elxvips.open( "/path/input.png" )
      %ImageFile{}

  """
  def open( path ) when is_binary( path ) do
    { :ok, %ImageFile{
      :path => path,
    } }
  end

  @doc """
  Will save the ImageFile in jpeg format to a specified path. Accepts quality and strip options.
  By default quality is set to 90 and strip to true.

  ## Examples

      iex> Elxvips.open( "/path/input.png" )
      iex> |> Elxvips.jpg( "/path/output.jpg", strip: true, quality: 72 ) 
      { :ok, %ImageFile{ :path => "/path/output.jpg", ... } }

  """
  def jpg( image_file, path, opts \\ [] )
  def jpg( image_file = %ImageFile{}, path, opts), do: to_jpg( image_file, path, opts )
  def jpg( { :ok, image_file = %ImageFile{} }, path, opts ), do: jpg( image_file, path, opts )

  @doc """
  Will save the ImageFile in png format to a specified path. Accepts quality and strip options.
  By default quality is set to 90, strip to true.

  ## Examples

      iex> Elxvips.open( "/path/input.jpg" )
      iex> |> Elxvips.png( "/path/output.png", strip: true, quality: 72 ) 
      { :ok, %ImageFile{ :path => "/path/output.png", ... } }
  """
  def png( image_file, path, opts \\ [] )
  def png( image_file = %ImageFile{}, path, opts ), do: to_png( image_file, path, opts )
  def png( { :ok, image_file }, path, opts ), do: to_png( image_file, path, opts )

  @doc """
  Same as jpg() but works with %ImageBytes{}.

  ## Examples

      iex> Elxvips.open( "/path/input.png" )
      iex> |> Elxvips.jpg_bytes( strip: true, quality: 72 ) 
      { :ok, %ImageBytes{ :bytes => [...] } }

  """
  def jpg_bytes( image_bytes, opts \\ [] )
  def jpg_bytes( image_bytes = %ImageBytes{}, opts ), do: to_jpg( image_bytes, opts )
  def jpg_bytes( { :ok, image_bytes = %ImageBytes{} }, opts ), do: jpg_bytes( image_bytes, opts )

  @doc """
  Same as png() but works with %ImageBytes{}.

  ## Examples

      iex> Elxvips.open( "/path/input.jpg" )
      iex> |> Elxvips.png_bytes( strip: true, quality: 72 ) 
      { :ok, %ImageBytes{ :bytes => [...] } }

  """
  def png_bytes( image_bytes, opts \\ [] )
  def png_bytes( image_bytes = %ImageBytes{}, opts ), do: to_png( image_bytes, opts )
  def png_bytes( { :ok, image_bytes = %ImageBytes{} }, opts ), do: to_png( image_bytes, opts )

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
  Returns dimensions of the specified image, works with a image path or bytes.

  ## Examples
      iex> Elxvips.open( "test/input.png" )
      iex> |> Elxvips.get_image_sizes()
      {:ok, [640, 486]}
  """
  def get_image_sizes( path ) when is_binary( path ), do: vips_get_image_sizes( path )
  def get_image_sizes( bytes ) when is_list( bytes ), do: vips_get_image_bytes_sizes( bytes )

  def get_image_sizes( %ImageFile{ :path => path } ), do: get_image_sizes( path )
  def get_image_sizes( {:ok, %ImageFile{ :path => path } } ), do: get_image_sizes( path )

  def get_image_sizes( %ImageBytes{ :bytes => bytes } ) when is_list( bytes ), do: get_image_sizes( bytes )
  def get_image_sizes( %ImageBytes{ :path => path } ) when is_binary( path ), do: get_image_sizes( path )
  def get_image_sizes( { :ok, image_bytes = %ImageBytes{} } ), do: get_image_sizes( image_bytes )

  @doc """
  Converts %ImageFile{} to %ImageBytes{} for further processing.

  ## Examples
      iex> Elxvips.open( "test/input.png" )
      iex> |> Elxvips.as_bytes()
      {:ok, %ImageBytes{}}
  """
  def as_bytes( %ImageFile{ :path => path, :resize => resize } ) do
    { :ok, %ImageBytes{
      :path => path,
      :resize => resize,
    } }
  end
  def as_bytes( { :ok, image_file = %ImageFile{} } ), do: as_bytes( image_file )

end