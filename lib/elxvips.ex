defmodule Elxvips.ResizeOptions do
  defstruct [
    width: :auto,
    height: :auto,
    resize_type: :auto, # :type conflicts with rust keyword "type"
  ]
end

defmodule Elxvips.SaveOptions do
  defstruct [
    quality: 90,
    format: :auto,
    strip: true,
    path: "",
    compression: 6,
    background: [],
  ]
end

defmodule Elxvips.ImageFile do
  defstruct [
    path: :nil,
    resize: %Elxvips.ResizeOptions{
      :width => 0,
      :height => 0,
      :resize_type => :auto,
    },
    save: %Elxvips.SaveOptions{},
    pdf: false,
    page: 0,
    n: 1,
  ]
end

defmodule Elxvips.ImageBytes do
  defstruct [
    bytes: nil,
    resize: %Elxvips.ResizeOptions{
      :width => 0,
      :height => 0,
      :resize_type => :auto,
    },
    save: %Elxvips.SaveOptions{},
    pdf: false,
    page: 0,
    n: 1,
  ]
end

defmodule Elxvips do
  @moduledoc """
  Documentation for `Elxvips`.
  """
  alias Elxvips.ImageFile, as: ImageFile
  alias Elxvips.ImageBytes, as: ImageBytes
  alias Elxvips.SaveOptions, as: SaveOptions

  # creating new image from an existing image path
  defp process_to_file( image_file = %ImageFile{}, path ) when is_binary( path ) do
    image_file = %ImageFile{ image_file |
      :save => Kernel.struct( image_file.save, [ path: path ] )
    }
    with :ok <- Elxvips.Native.vips_process_file_to_file( image_file ) do
      { :ok, %ImageFile{
        :path => path,
      } }
    else
      err -> err
    end
  end
  defp process_to_file( image_bytes = %ImageBytes{ :bytes => bytes }, path ) when is_binary( path ) and is_bitstring( bytes ) do
    image_bytes = %ImageBytes{ image_bytes |
      :save => Kernel.struct( image_bytes.save, [ path: path ] )
    }
    with :ok <- Elxvips.Native.vips_process_bytes_to_file( image_bytes ) do
      { :ok, %ImageFile{
        :path => path,
      } }
    else
      err -> err
    end
  end
  # In case the we have a image path as image_bytes
  defp process_to_bytes( image_file = %ImageFile{ :path => path } ) when is_binary( path )  do
    with { :ok, bytes } <- Elxvips.Native.vips_process_file_to_bytes( image_file ) do
      { :ok, %ImageBytes{
        :bytes => bytes,
      } }
    else
      err -> err
    end
  end
  # In case we have raw bytes
  defp process_to_bytes( image_bytes = %ImageBytes{ :bytes => bytes } ) when is_bitstring( bytes ) do
    with { :ok, bytes } <- Elxvips.Native.vips_process_bytes_to_bytes( image_bytes ) do
      { :ok, %ImageBytes{
        :bytes => bytes,
      } }
    else
      err -> err
    end
  end


  defp format_merge( :background, a_val, b_val ) do
    case b_val do
      [] -> a_val
      _ -> b_val
    end
  end

  defp format_merge( _, _, b_val ) do
    b_val
  end

  defp format( image_file = %ImageFile{}, format , opts ) do
    with opts when is_list( opts ) <- check_opts( opts ) do
      { :ok, %ImageFile{ image_file |
        :save => Map.merge( image_file.save, Map.new( opts ++ [ format: format, path: "" ] ), &format_merge/3 ),
      } }
    end
  end

  defp format( image_bytes = %ImageBytes{}, format , opts ) do
    with opts when is_list( opts ) <- check_opts( opts ) do
      { :ok, %ImageBytes{ image_bytes |
        :save => Map.merge( image_bytes.save, Map.new( opts ++ [ format: format, path: "" ] ), &format_merge/3 ),
      } }
    end

  end
  defp format( { :ok, image }, format, opts ), do: format( image, format, opts )

  def check_opts( opts \\ [] ) do
    with opts when is_list( opts ) <- check_background( opts )  do
      opts
    end
  end

  defp check_background( opts ) do
    with background_list when is_list( background_list ) <- background_opts( Keyword.get( opts, :background, [] ) ) do
      Keyword.put( opts, :background, background_list )
    end
  end

  # vips expects a vector of f64, this should convert integers to float
  defp background_opts( [ c1 ] ), do: [ c1 / 1 ]
  defp background_opts( [ c1, c2, c3 ] ), do: [ c1 / 1, c2 / 1, c3 / 1 ]
  defp background_opts( [] ), do: []
  defp background_opts( _ ), do: { :error, "background can be a vector of 1 or 3 numbers" }


  @doc """
  Sets the background of the image in case there is a transparent background.
  Accepts a empty list, or a list of length 2 or 3.

  ## Examples
      iex> import Elxvips
      iex>
      iex> from_file( "test/input.png" )
      iex  |> jpg()
      iex> |> background( [ 255, 0, 0 ] ) # red background
      iex  |> to_file( "test/output.jpg" )
      {:ok, %ImageBytes{}}

  """
  # def background( image = %ImageFile{}, c1 ), do: background( image, c1 )
  # def background( image = %ImageFile{}, c1, c2, c3 ), do: background( image, c1 )
  # def background( image = %ImageFile{}, c1 ), do: background( image, c1 )
  # def background( image = %ImageFile{}, c1, c2, c3 ), do: background( image, c1 )

  def background( image, opts \\ [] )

  def background( image_file = %ImageFile{}, colors ) do
    with background_list when is_list( background_list ) <- background_opts( colors ) do
      { :ok, %ImageFile{ image_file |
        :save => %SaveOptions { image_file.save |
          :background => background_list,
        },
      } }
    end
  end

  def background( image_bytes = %ImageBytes{}, colors ) do
    with background_list when is_list( background_list ) <- background_opts( colors ) do
      { :ok, %ImageBytes{ image_bytes |
        :save => %SaveOptions { image_bytes.save |
          :background => background_list,
        },
      } }
    end
  end

  def background( { :ok, image_file = %ImageFile{} }, colors ), do: background( image_file, colors )
  def background( { :ok, image_bytes = %ImageBytes{} }, colors ), do: background( image_bytes, colors )

  @doc """
  Applies resize options to an %ImageFile{} or %ImageBytes{}, accepts :width, :height and :type (not implemented yet).
  If no width or height is specified dimensions are calculated from the input image.
  Empty resize( no :width and no :height) will produce an image with the dimensions as the original one.

  ## Examples
      iex> import Elxvips
      iex>
      iex> from_file( "test/input.png" )
      iex> |> resize( width: 300 )
      iex  |> to_bytes()
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

  @save_opts_default [ quality: 100, strip: true, compression: 6, background: [] ]

  @jpg_default_opts Keyword.merge( @save_opts_default, [ quality: 90 ] )
  @doc """
  Will save the ImageFile in jpeg format to a specified path. Accepts quality and strip options.
  By default quality is set to 90 and strip to true.

  ## Examples
      iex> import Elxvips
      iex>
      iex> from_file( "/path/input.png" )
      iex> |> jpg( strip: true, quality: 72 )
      iex  |> to_file( "/path/output.jpg" )
      { :ok, %ImageFile{} }
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
      iex> import Elxvips
      iex>
      iex> from_file( "/path/input.jpg" )
      iex> |> png( strip: true, quality: 72 )
      iex  |> to_file( "/path/output.png" )
      { :ok, %ImageFile{} }
  """
  def png( image, opts \\ [] )
  def png( image_file = %ImageFile{}, opts ), do: format( image_file, :png, Keyword.merge( @png_default_opts, opts ) )
  def png( image_file = %ImageBytes{}, opts ), do: format( image_file, :png, Keyword.merge( @png_default_opts, opts ) )
  def png( { :ok, image }, opts ), do: png( image, opts )

  @webp_default_opts Keyword.merge( @save_opts_default, [ quality: 100 ] )
  @doc """
  Will save the ImageFile in webp format to a specified path. Accepts quality and strip options.
  By default quality is set to 100, and strip to true.

  ## Examples
      iex> import Elxvips
      iex>
      iex> from_file( "/path/input.jpg" )
      iex> |> webp( strip: true, quality: 72 )
      iex  |> to_file( "/path/output.webp" )
      { :ok, %ImageFile{} }
  """
  def webp( image, opts \\ [] )
  def webp( image_file = %ImageFile{}, opts ), do: format( image_file, :webp, Keyword.merge( @webp_default_opts, opts ) )
  def webp( image_file = %ImageBytes{}, opts ), do: format( image_file, :webp, Keyword.merge( @webp_default_opts, opts ) )
  def webp( { :ok, image }, opts ), do: webp( image, opts )

  @avif_default_opts Keyword.merge( @save_opts_default, [ quality: 100 ] )
@doc """
  Will save the ImageFile in avif(AV1) format to a specified path. Accepts quality and strip options.
  By default quality is set to 100

  ## Examples
      iex> import Elxvips
      iex>
      iex> from_file( "/path/input.jpg" )
      iex> |> avif( quality: 72 )
      iex  |> to_file( "/path/output.avif" )
      { :ok, %ImageFile{} }
  """
  def avif( image, opts \\ [] )
  def avif( image_file = %ImageFile{}, opts ), do: format( image_file, :avif, Keyword.merge( @avif_default_opts, opts ) )
  def avif( image_file = %ImageBytes{}, opts ), do: format( image_file, :avif, Keyword.merge( @avif_default_opts, opts ) )
  def avif( { :ok, image }, opts ), do: avif( image, opts )

  @doc """
  Will create an %ImageFile{} struct from path. This struct will be used for further processing.

  ## Examples
      iex> import Elxvips
      iex>
      iex> from_file( "/path/input.png" )
      %ImageFile{}

  """
  def from_file( path ) when is_binary( path ) do
    { :ok, %ImageFile{
      :path => path,
    } }
  end
  def from_file( { :ok, %ImageFile{} = image_file } ) do
    { :ok, image_file }
  end

  @doc """
  Will create an %ImageFile{} struct from a pdf path. This struct will be used for further processing.
  Accepts the following options:
  * `:page` - page number to extract from pdf, default is 0
  * `:n` - number of pages to extract from pdf, default is 1

  ## Examples
      iex> import Elxvips
      iex>
      iex> from_pdf( "/path/input.pdf", page: 0, n: 2 )
      %ImageFile{}
  """

  def from_pdf( path, opts \\ [ page: 0 ] ) when is_binary( path ) do
    page = Keyword.get( opts, :page, 0 )
    n = Keyword.get( opts, :n, 1 )

    { :ok, %ImageFile{
      :path => path,
      :pdf => true,
      :page => page,
      :n => n,
    } }
    |> jpg()
  end

  @doc """
  Will create an %ImageByte{} struct from bitstring or byte list. This struct will be used for further processing.

  ## Examples
      iex> import Elxvips
      iex>
      iex> file = File.open!( "/path/input.png" )
      iex> bytes = IO.binread( file, :all )
      iex> from_bytes( bytes )
      %ImageBytes{}

  """
  def from_bytes( { :ok, %ImageBytes{} = image_bytes } ) do
    { :ok, image_bytes }
  end
  def from_bytes( bytes ) when is_bitstring( bytes ) do
    { :ok, %ImageBytes{
      :bytes => bytes,
    } }
  end

  @doc """
  Will create an %ImageByte{} struct from pdf bitstring or byte list. This struct will be used for further processing.
  Accepts the following options:
  * `:page` - page number to extract from pdf, default is 0
  * `:n` - number of pages to extract from pdf, default is 1

  ## Examples
      iex> import Elxvips
      iex>
      iex> file = File.open!( "/path/input.pdf" )
      iex> bytes = IO.binread( file, :all )
      iex> from_pdf_bytes( bytes, page: 0, n: 2 )
      %ImageBytes{}

  """
  def from_pdf_bytes( bytes ), do: from_pdf_bytes( bytes, [] )
  def from_pdf_bytes( bytes, opts ) when is_bitstring( bytes ) do
    page = Keyword.get( opts, :page, 0 )
    n = Keyword.get( opts, :n, 1 )

    { :ok, %ImageBytes{
      :bytes => bytes,
      :pdf => true,
      :page => page,
      :n => n
    } }
    |> jpg()
  end
  def from_pdf_bytes( bytes, opts ) when is_bitstring( bytes ) do
    page = Keyword.get( opts, :page, 0 )
    n = Keyword.get( opts, :n, 1 )

    { :ok, %ImageBytes{
      :bytes => bytes,
      :pdf => true,
      :page => page,
      :n => n
    } }
    |> jpg()
  end

  @doc """
  Will create a new %ImageBytes{} struct containing all the changes.

  ## Examples
      iex> import Elxvips
      iex>
      iex> from_file( "test/input.png" )
      iex> |> png()
      iex> |> to_bytes()
      {:ok, %ImageBytes{}}
  """
  def to_bytes( image = %ImageFile{} ), do: process_to_bytes( image )
  def to_bytes( image = %ImageBytes{} ), do: process_to_bytes( image )
  def to_bytes( { :ok, image } ), do: to_bytes( image )

  @doc """
  Will save the image to a path on disk and return a new %ImageFile{} from the new path.

  ## Examples
      iex> import Elxvips
      iex>
      iex> from_file( "test/input.png" )
      iex> |> png()
      iex> |> to_file( "test/outping.png" )
      {:ok, %ImageBytes{}}
  """
  def to_file( image = %ImageFile{}, path ) when is_binary( path ), do: process_to_file( image, path )
  def to_file( image = %ImageBytes{}, path ) when is_binary( path ), do: process_to_file( image, path )
  def to_file( { :ok, image }, path ), do: to_file( image, path )

  def set_concurrency( concurrency ) when is_integer( concurrency ) do
    Elxvips.Native.set_concurrency( concurrency )
  end

  @doc """
  Returns dimensions of the specified image, works with a image path or bytes.

  ## Examples
      iex> import Elxvips
      iex>
      iex> from_file( "test/input.png" )
      iex> |> get_image_sizes()
      {:ok, [640, 486]}
  """
  def get_image_sizes( %ImageFile{ :path => path } ), do: Elxvips.Native.vips_get_image_sizes( path )
  def get_image_sizes( {:ok, image_file = %ImageFile{ :path => path } } ) when is_binary( path ), do: get_image_sizes( image_file )

  def get_image_sizes( %ImageBytes{ :bytes => bytes } ) when is_bitstring( bytes ), do: Elxvips.Native.vips_get_image_bytes_sizes( bytes )
  def get_image_sizes( { :ok, image_bytes = %ImageBytes{} } ), do: get_image_sizes( image_bytes )

  @doc """
  Returns format of the specified image, works with a image path or bytes.

  ## Examples
      iex> import Elxvips
      iex>
      iex> from_file( "test/input.png" )
      iex> |> get_image_format()
      {:ok, :png}
  """
  def get_image_format( %ImageFile{ :path => path } ), do: Elxvips.Native.vips_get_image_file_format( path )
  def get_image_format( {:ok, image_file = %ImageFile{} } ), do: get_image_format( image_file )

  def get_image_format( %ImageBytes{ :bytes => bytes } ), do: Elxvips.Native.vips_get_image_bytes_format( bytes )
  def get_image_format( { :ok, image_bytes = %ImageBytes{} } ), do: get_image_format( image_bytes )

end
