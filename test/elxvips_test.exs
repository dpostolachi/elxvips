defmodule ElxvipsTest do
  use ExUnit.Case
  import Elxvips
  # doctest Elxvips.Libvips

  test "Resize" do
    # assert :ok = resize( "test/input.png", "./test/output.jpg" )
  end

  test "Resize options" do
    result = open( "test/input.png" )
    |> resize( height: 300, width: 250 )
    |> jpg( "test/output.jpg", quality: 100 )

    assert :ok = result
  end
end