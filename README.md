# Meow

Print ASCII cats to your terminal!

This is a simple command-line tool to display cute little kitties :D

I love cats

## Usage

```
Usage: meow [OPTIONS]

Options:
  -c, --count <COUNT>  How many cats to print [default: 1]
  -l, --literally      Are you literally this cat?
  -h, --help           Print help
  -V, --version        Print version
```

## Installation

### Ubuntu / Debian / etc

This package is available through apt. Run:

```sh
sudo apt update
sudo apt install meow
```

Note: the Debian package is currently being rolled out and may not be available on all systems.
Currently available in: Ubuntu 25.04, Debian Testing.

### Nix / Nixos

Try the package out by running `nix-shell -p meow`.

If you want to add it to your configuration, you can add the following to your configuration:

```nix
environment.systemPackages = with pkgs; [
  meow
];
```

Or, if you are using Home Manager:

```nix
home.packages = with pkgs; [
  meow
];
```

### From Cargo

To get the package directly from the Rust / Cargo archives, run:

```sh
cargo install meow-cli
```

The binary will then be built to some directory that will be output to your command line.

## Building from source

1. Install Rust
1. Clone this repository
1. Build and run with `cargo run` or `cargo run -- [OPTIONS]`
