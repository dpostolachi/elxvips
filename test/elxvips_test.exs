defmodule ElxvipsTest do
  use ExUnit.Case
  import Elxvips
  # doctest Elxvips.Libvips

  test "Set concurreny" do
    assert :ok = set_concurrency( 4 )
  end

  test "Resize png > jpg" do
    result = open( "test/input.png" )
    |> resize( width: 250, height: 300 )
    |> jpg( "test/output.jpg", quality: 60 )
    |> get_image_sizes()

    assert result == { :ok, [ 250, 300 ] }
  end

  test "Resize png > png" do
    result = open( "test/input.png" )
    |> resize( width: 300, height: 250 )
    |> png( "test/output.png", quality: 60 )
    |> get_image_sizes()

    assert result == { :ok, [ 300, 250 ] }
  end

  test "Original size" do
    input_image = open( "test/input.png" )
    { :ok, [ input_width, input_height ] } = get_image_sizes( input_image )

    { :ok, [ output_width, output_height ] } = input_image
    |> png( "test/original.png", quality: 60 )
    |> get_image_sizes()

    assert [ input_width, input_height ] == [ output_width, output_height ]
  end

  test "Vertical image > horizontal" do
    result = open( "test/vertical.jpg" )
    |> resize( width: 200, height: 100 )
    |> jpg( "test/vertical_1.png", quality: 60 )
    |> get_image_sizes()

    assert result == { :ok, [ 200, 100 ] }
  end

  test "Vertical image > vertical" do
    result = open( "test/vertical.jpg" )
    |> resize( width: 100, height: 200 )
    |> jpg( "test/vertical_2.png", quality: 60 )
    |> get_image_sizes()

    assert result == { :ok, [ 100, 200 ] }
  end

  test "To bytes" do
    bytes = to_bytes( "test/vertical.jpg" )
    IO.puts "#{ inspect( bytes ) }"
  end

end