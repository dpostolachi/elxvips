defmodule ElxvipsTest do
  use ExUnit.Case
  # doctest Elxvips.Libvips

  test "Resize" do
    assert :ok = Elxvips.resize( "test/input.jpg", "./test/output.jpg" )
  end
end