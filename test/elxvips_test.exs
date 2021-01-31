defmodule ElxvipsTest do
  use ExUnit.Case
  import Elxvips
  # doctest Elxvips.Libvips

  test "Set concurreny" do
    assert :ok = set_concurrency( 8 )
  end

  test "Resize png > jpg" do
    result = from_file( "test/input.png" )
    |> resize( width: 250, height: 300 )
    |> jpg( quality: 60 )
    |> to_bytes()
    |> get_image_sizes()

    assert result == { :ok, [ 250, 300 ] }
  end

  test "Resize png > png" do
    result = from_file( "test/input.png" )
    |> resize( width: 300, height: 250 )
    |> png( quality: 60 )
    |> to_bytes()
    |> get_image_sizes()

    assert result == { :ok, [ 300, 250 ] }
  end

  test "Original size with bytes" do
    input_image = from_file( "test/input.png" )
    { :ok, [ input_width, input_height ] } = get_image_sizes( input_image )

    { :ok, [ output_width, output_height ] } = input_image
    |> png( quality: 60 )
    |> to_bytes()
    |> get_image_sizes()

    assert [ input_width, input_height ] == [ output_width, output_height ]
  end

  test "Vertical image > horizontal" do
    result = from_file( "test/vertical.jpg" )
    |> resize( width: 200, height: 100 )
    |> jpg( quality: 60 )
    |> to_bytes()
    |> get_image_sizes()

    assert result == { :ok, [ 200, 100 ] }
  end

  test "Vertical image > vertical" do
    result = from_file( "test/vertical.jpg" )
    |> resize( width: 100, height: 200 )
    |> jpg( quality: 60 )
    |> to_bytes()
    |> get_image_sizes()

    assert result == { :ok, [ 100, 200 ] }
  end

  test "Image > bytes > bytes" do
    result = from_file( "test/vertical.jpg" )
    |> resize( height: 150, width: 150 )
    |> jpg( strip: false )
    |> to_bytes()
    |> resize( width: 100, height: 120 )
    |> png()
    |> to_bytes()
    |> get_image_sizes()

    assert result == { :ok, [ 100, 120 ] }
  end

  test "8K Image" do
    result = from_file( "test/8k.jpg" )
    |> resize( height: 720 )
    |> jpg( strip: true )
    |> to_bytes()
    |> get_image_sizes()
    assert result == { :ok, [ 1080, 720 ] }
  end

  test "from file bytes" do
    file = File.open!( "test/input.png", [ :read ] )
    bytes = IO.binread( file, :all )

    result = from_bytes( bytes )
    |> resize( width: 100, height: 100 )
    |> png()
    |> to_bytes()

    sizes = result
    |> get_image_sizes()

    assert sizes == { :ok, [ 100, 100 ] }

    image_file_sizes = result
    |> png()
    |> to_file( "test/from_bytes.png" )
    |> get_image_sizes()

    assert sizes == image_file_sizes
  end

  test "from file bytes, autodetect format" do

    file = File.open!( "test/input.png", [ :read ] )
    bytes = IO.binread( file, :all )

    sizes = from_bytes( bytes )
    |> resize( width: 100, height: 100 )
    |> to_bytes()
    |> get_image_sizes()

    assert sizes == { :ok, [ 100, 100 ] }

  end
  test "from file, autodetect format" do

    format = from_file( "test/input.png" )
    |> resize( width: 100, height: 100 )
    |> to_bytes()
    |> get_image_format()

    assert format == { :ok, :png }

  end

  test "from png to webp" do

    format = from_file( "test/input.png" )
    |> resize( width: 100, height: 100 )
    |> webp()
    |> to_bytes()
    |> get_image_format()

    assert format == { :ok, :webp }

  end

  test "from png to jpg, transparent background" do

    format = from_file( "test/input2.png" )
    |> resize( width: 100, height: 100 )
    |> background( [ 255, 0, 255 ] )
    |> jpg()
    |> to_file( "test/output2.jpg" )
    |> get_image_format()

    assert format == { :ok, :jpg }

  end

end
