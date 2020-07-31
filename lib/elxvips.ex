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
    compression: 6,
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
  defp vips_process_file_to_file(_a), do: :erlang.nif_error(:nif_not_loaded) # applies processing from %ImageFile{}
  defp vips_process_file_to_bytes(_a), do: :erlang.nif_error(:nif_not_loaded) # applies processing from %ImageBytes{} created from image path
  defp vips_process_bytes_to_bytes(_a), do: :erlang.nif_error(:nif_not_loaded) # applies processing from %ImageBytes{} created from image bytes
  defp vips_process_bytes_to_file(_a), do: :erlang.nif_error(:nif_not_loaded) # applies processing from %ImageBytes{} created from image bytes

  # creating new image from an existing image path
  defp process_to_file( image_file = %ImageFile{}, path ) when is_binary( path ) do
    image_file = %ImageFile{ image_file |
      :save => Kernel.struct( image_file.save, [ path: path ] )
    }
    with :ok <- vips_process_file_to_file( image_file ) do
      { :ok, %ImageFile{
        :path => path,
      } }
    else
      err -> err
    end
  end
  defp process_to_file( image_bytes = %ImageBytes{ :bytes => bytes }, path ) when is_binary( path ) and is_list( bytes ) do
    image_bytes = %ImageBytes{ image_bytes |
      :save => Kernel.struct( image_bytes.save, [ path: path ] )
    }
    with :ok <- vips_process_bytes_to_file( image_bytes ) do
      { :ok, %ImageFile{
        :path => path,
      } }
    else
      err -> err
    end
  end
  # In case the we have a image path as image_bytes
  defp process_to_bytes( image_file = %ImageFile{ :path => path } ) when is_binary( path )  do
    with { :ok, bytes } <- vips_process_file_to_bytes( image_file ) do
      { :ok, %ImageBytes{
        :bytes => bytes,
      } }
    else
      err -> err
    end
  end
  # In case we have raw bytes
  defp process_to_bytes( image_bytes = %ImageBytes{ :bytes => bytes } ) when is_list( bytes ) do
    with { :ok, bytes } <- vips_process_bytes_to_bytes( image_bytes ) do
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
  
  ## Examples
      iex> Elxvips.from_file( "test/input.png" )
      iex> |> Elxvips.resize( width: 300 )
      {:ok, %ImageBytes{}}

      iex> Elxvips.from_file( "test/input.png" )
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

  defp format( image_file = %ImageFile{}, format , opts ) do
    { :ok, %ImageFile{ image_file |
      :save => Kernel.struct( %SaveOptions{}, opts ++ [ format: format, path: "" ] ),
    } }
  end
  defp format( image_file = %ImageBytes{}, format , opts ) do
    { :ok, %ImageBytes{ image_file |
      :save => Kernel.struct( %SaveOptions{}, opts ++ [ format: format, path: "" ] ),
    } }
  end
  defp format( { :ok, image }, format, opts ), do: format( image, format, opts )

  @save_opts_default [ quality: 100, strip: true, compression: 6 ]

  @jpg_default_opts Keyword.merge( @save_opts_default, [ quality: 90 ] )
  @doc """
  Will save the ImageFile in jpeg format to a specified path. Accepts quality and strip options.
  By default quality is set to 90 and strip to true.

  ## Examples

      iex> Elxvips.open( "/path/input.png" )
      iex> |> Elxvips.jpg( "/path/output.jpg", strip: true, quality: 72 ) 
      { :ok, %ImageFile{ :path => "/path/output.jpg", ... } }

  """
  def jpg( image, opts \\ [] )
  def jpg( image_file = %ImageFile{}, opts ), do: format( image_file, :jpg, Keyword.merge( @jpg_default_opts, opts ) )
  def jpg( image_file = %ImageBytes{}, opts ), do: format( image_file, :jpg, Keyword.merge( @jpg_default_opts, opts ) )
  def jpg( { :ok, image }, opts ), do: jpg( image, opts )

  @png_default_opts Keyword.merge( @save_opts_default, [ quality: 100 ] )
  @doc """
  Will save the ImageFile in png format to a specified path. Accepts quality, compression(0-9) and strip options.
  By default quality is set to 100, compression to 6, strip to true. Decreasing compression will speed up image saving.

  ## Examples

      iex> Elxvips.open( "/path/input.jpg" )
      iex> |> Elxvips.png( "/path/output.png", strip: true, quality: 72 ) 
      { :ok, %ImageFile{ :path => "/path/output.png", ... } }
  """
  def png( image, opts \\ [] )
  def png( image_file = %ImageFile{}, opts ), do: format( image_file, :png, Keyword.merge( @png_default_opts, opts ) )
  def png( image_file = %ImageBytes{}, opts ), do: format( image_file, :png, Keyword.merge( @png_default_opts, opts ) )
  def png( { :ok, image }, opts ), do: png( image, opts )

  @doc """
  Will create an %ImageFile{} struct from path. This struct will be used for further processing.

  ## Examples

      iex> Elxvips.from_file( "/path/input.png" )
      %ImageFile{}

  """
  def from_file( path ) when is_binary( path ) do
    { :ok, %ImageFile{
      :path => path,
    } }
  end

  @doc """
  Will create an %ImageByte{} struct from bitstring or byte list. This struct will be used for further processing.

  ## Examples

      iex> file = File.open!( "/path/input.png" )
      iex> bytes = IO.binread( file, :all )
      iex> Elxvips.from_bytes( bytes )
      %ImageBytes{}

  """
  def from_bytes( bytes ) when is_list( bytes ) do
    { :ok, %ImageBytes{
      :bytes => bytes,
    } }
  end
  def from_bytes( bytes ) when is_bitstring( bytes ) do
    { :ok, %ImageBytes{
      :bytes => :erlang.binary_to_list( bytes ),
    } }
  end

  @doc """
  Will create a new %ImageBytes{} struct containing all the changes.

  ## Examples
      iex> Elxvips.from_file( "test/input.png" )
      iex> |> Elxvips.png()
      iex> |> Elxvips.to_bytes()
      {:ok, %ImageBytes{}}
  """
  def to_bytes( image = %ImageFile{} ), do: process_to_bytes( image )
  def to_bytes( image = %ImageBytes{} ), do: process_to_bytes( image )
  def to_bytes( { :ok, image } ), do: to_bytes( image )

  @doc """
  Will save the image to a path on disk and return a new %ImageFile{} from the new path.

  ## Examples
      iex> Elxvips.from_file( "test/input.png" )
      iex> |> Elxvips.png()
      iex> |> Elxvips.to_file( "test/outping.png" )
      {:ok, %ImageBytes{}}
  """
  def to_file( image = %ImageFile{}, path ) when is_binary( path ), do: process_to_file( image, path )
  def to_file( image = %ImageBytes{}, path ) when is_binary( path ), do: process_to_file( image, path )
  def to_file( { :ok, image }, path ), do: to_file( image, path )

  def set_concurrency( concurrency ) when is_integer( concurrency ) do
    vips_set_concurrency( concurrency )
  end

  @doc """
  Returns dimensions of the specified image, works with a image path or bytes.

  ## Examples
      iex> Elxvips.from_file( "test/input.png" )
      iex> |> Elxvips.get_image_sizes()
      {:ok, [640, 486]}
  """
  def get_image_sizes( path ) when is_binary( path ), do: vips_get_image_sizes( path )
  def get_image_sizes( bytes ) when is_list( bytes ), do: vips_get_image_bytes_sizes( bytes )

  def get_image_sizes( %ImageFile{ :path => path } ), do: get_image_sizes( path )
  def get_image_sizes( {:ok, %ImageFile{ :path => path } } ), do: get_image_sizes( path )

  def get_image_sizes( %ImageBytes{ :bytes => bytes } ) when is_list( bytes ), do: get_image_sizes( bytes )
  def get_image_sizes( { :ok, image_bytes = %ImageBytes{} } ), do: get_image_sizes( image_bytes )

end