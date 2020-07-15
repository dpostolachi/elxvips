defmodule Elxvips.MixProject do
  use Mix.Project

  def project do
    [
      app: :elxvips,
      version: "0.0.1",
      elixir: "~> 1.10",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      compilers: [:rustler] ++ Mix.compilers(),
      rustler_crates: rustler_crates(),
      description: description(),
      package: package(),
    ]
  end

  defp package do
    [
        name: "elxvips",
        files: [ "lib", "mix.exs", "native/**/*.rs", "native/**/Cargo.lock", "native/**/Cargo.toml", "README.md" ],
        licenses: [ "MIT" ],
        links: %{
            "GitHub" => "https://github.com/dpostolachi/elxvips"
        },
        maintainers: [ "dpostolachi" ]
    ]
  end

  defp description do
    "Experimental bindings to libVips through rustler"
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      # {:dep_from_hexpm, "~> 0.3.0"},
      # {:dep_from_git, git: "https://github.com/elixir-lang/my_dep.git", tag: "0.1.0"}
      {:rustler, ">= 0.21.1"},
      {:ex_doc, ">= 0.0.0", only: :dev, runtime: false}
    ]
  end

  defp rustler_crates do
    [
      lvips: [path: "native/lvips", mode: if(Mix.env() == :prod, do: :release, else: :debug)]
    ]
  end
end
