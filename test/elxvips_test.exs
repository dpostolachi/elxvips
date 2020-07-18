defmodule ElxvipsTest do
  use ExUnit.Case
  import Elxvips
  # doctest Elxvips.Libvips

  test "Set concurreny" do
    assert :ok = set_concurrency( 4 )
  end

  test "Resize png > jpg" do
    result = open( "test/input.png" )
    |> as_bytes()
    |> resize( width: 250, height: 300 )
    |> jpg_bytes( quality: 60 )
    |> get_image_sizes()

    assert result == { :ok, [ 250, 300 ] }
  end

  test "Resize png > png" do
    result = open( "test/input.png" )
    |> as_bytes()
    |> resize( width: 300, height: 250 )
    |> png_bytes( quality: 60 )
    |> get_image_sizes()

    assert result == { :ok, [ 300, 250 ] }
  end

  test "Original size with bytes" do
    input_image = open( "test/input.png" )
    { :ok, [ input_width, input_height ] } = get_image_sizes( input_image )

    { :ok, [ output_width, output_height ] } = input_image
    |> as_bytes()
    |> png_bytes( quality: 60 )
    |> get_image_sizes()

    assert [ input_width, input_height ] == [ output_width, output_height ]
  end

  test "Vertical image > horizontal" do
    result = open( "test/vertical.jpg" )
    |> resize( width: 200, height: 100 )
    |> as_bytes()
    |> jpg_bytes( quality: 60 )
    |> get_image_sizes()

    assert result == { :ok, [ 200, 100 ] }
  end

  test "Vertical image > vertical" do
    result = open( "test/vertical.jpg" )
    |> as_bytes()
    |> resize( width: 100, height: 200 )
    |> jpg_bytes( quality: 60 )
    |> get_image_sizes()

    assert result == { :ok, [ 100, 200 ] }
  end

  test "Image > bytes > bytes" do
    result = open( "test/vertical.jpg" )
    |> as_bytes()
    |> resize( height: 150, width: 150 )
    |> jpg_bytes( strip: false )
    |> resize( width: 100, height: 120 )
    |> png_bytes()
    |> get_image_sizes()

    assert result == { :ok, [ 100, 120 ] }
  end

  test "8K Image" do
    result = open( "test/8k.jpg" )
    |> as_bytes()
    |> resize( height: 720 )
    |> jpg_bytes( strip: true )
    |> get_image_sizes()

    assert result == { :ok, [ 1080, 720 ] }
  end

end