defmodule ElxvipsTest do
  use ExUnit.Case
  import Elxvips
  # doctest Elxvips.Libvips

  test "Resize png > jpg" do
    result = open( "test/input.png" )
    |> resize( height: 300, width: 250 )
    |> jpg( "test/output.jpg", quality: 100 )

    assert :ok = result
  end

  test "Resize png > png" do
    result = open( "test/input.png" )
    |> resize( height: 300, width: 250 )
    |> png( "test/output.png", quality: 100 )

    assert :ok = result
  end
end