defmodule Elxvips.Native do
	version = Mix.Project.config()[:version]
	source_url = Mix.Project.config()[:source_url]
  
	use RustlerPrecompiled,
	  otp_app: :elxvips,
	  crate: "elxvips",
	  base_url:
		"#{ source_url }/releases/download/v#{version}",
	  force_build: System.get_env("ELXVIPS_BUILD") in ["1", "true"],
	  version: version
  
	# When your NIF is loaded, it will override this function.
	def add(_a, _b), do: :erlang.nif_error(:nif_not_loaded)
end