defmodule Elxvips.Native do
	version = Mix.Project.config()[:version]
	source_url = Mix.Project.config()[:source_url]

	use RustlerPrecompiled,
	  otp_app: :elxvips,
	  crate: "lvips",
		mode: :release,
	  base_url:
		"#{ source_url }/releases/download/v#{version}",
	  force_build: System.get_env("ELXVIPS_BUILD") in ["1", "true"],
	  version: version,
    targets: ~w(
      x86_64-unknown-linux-gnu
      x86_64-unknown-linux-musl
    )


  # NIFs
  def set_concurrency(_a), do: :erlang.nif_error(:nif_not_loaded)
  def vips_get_image_sizes(_a), do: :erlang.nif_error(:nif_not_loaded) # returns {:ok, { width, height } }
  def vips_get_image_bytes_sizes(_a), do: :erlang.nif_error(:nif_not_loaded) # same but works with bytes
  def vips_process_file_to_file(_a), do: :erlang.nif_error(:nif_not_loaded) # applies processing from %ImageFile{}
  def vips_process_file_to_bytes(_a), do: :erlang.nif_error(:nif_not_loaded) # applies processing from %ImageBytes{} created from image path
  def vips_process_bytes_to_bytes(_a), do: :erlang.nif_error(:nif_not_loaded) # applies processing from %ImageBytes{} created from image bytes
  def vips_process_bytes_to_file(_a), do: :erlang.nif_error(:nif_not_loaded) # applies processing from %ImageBytes{} created from image bytes
  def vips_get_image_file_format(_a), do: :erlang.nif_error(:nif_not_loaded) # applies processing from %ImageBytes{} created from image bytes
  def vips_get_image_bytes_format(_a), do: :erlang.nif_error(:nif_not_loaded) # applies processing from %ImageBytes{} created from image bytes

end
