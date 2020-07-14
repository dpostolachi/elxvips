defmodule Elxvips do
  @moduledoc """
  Documentation for `Elxvips`.
  """
  use Rustler, otp_app: :elxvips, crate: "lvips"

  @doc """
  Resize image.

  ## Examples

      iex> Elxvips.resize( "/path/input.jpg", "/path/output.jpg" )
      :ok

  """
  def resize(_a, _b), do: :erlang.nif_error(:nif_not_loaded)
end