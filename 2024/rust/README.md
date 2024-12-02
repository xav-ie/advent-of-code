# Advent of Code 2024 Solutions - Rust

Solutions to [Advent of Code 2024](https://adventofcode.com/2024) in Rust.

## Development Setup

1. Install `nix` ❄️. It is used to fetch the dependencies reproducibly. [Determinate Systems installer](https://github.com/determinateSystems/nix-installer?tab=readme-ov-file#determinate-nix-installer) is recommended for install.
2. Install `devenv`. `devenv` uses `nix` to manage dependencies, since managing `nix` is cumbersome. It basically adds superpowers to `nix`.
3. Install `direnv`, which is used to automatically load the the development environment.
4. Run `direnv allow`. It will now automatically load the development environment using `devenv`.

Steps 2 and 3 can be accomplished quickly by running:
```sh
nix shell nixpkgs#direnv nixpkgs#devenv
```
^ which drops you into a shell with `direnv` and `devenv` installed, temporarily. You will have to figure out how you want to install `direnv` and `devenv` more permanently. I recommend [Home Manager](https://github.com/nix-community/home-manager).

## Running the solutions

just read the `justfile` :)

## Thank you @ChristopherBiscardi for the Rust template setup :)

The benches and the fetching scripts are not mine. I will probably be stealing a lot of the extra bench solutions as well, because I want them in my archive for learning.
