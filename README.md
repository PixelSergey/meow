# Meow


Print ASCII cats to your terminal!

This is a simple command-line tool to display cute little kitties :D

I love cats

[![Number of repositories](https://repology.org/badge/tiny-repos/meow-ascii-cats.svg)](https://repology.org/project/meow-ascii-cats/versions)
<br />
[![latest packaged version(s)](https://repology.org/badge/latest-versions/meow-ascii-cats.svg)](https://repology.org/project/meow-ascii-cats/versions)
<br />
[![Packaging status](https://repology.org/badge/vertical-allrepos/meow-ascii-cats.svg)](https://repology.org/project/meow-ascii-cats/versions)


## Usage

```
Usage: meow [OPTIONS]

Options:
  -c, --count <COUNT>  How many cats to print [default: 1]
  -l, --literally      Are you literally this cat?
  -h, --help           Print help
  -V, --version        Print version
```

## Packaging shenanigans

The goal of this package is to eventually be in every Linux package repository and more (who doesn't like cats?)
You can see the progress here: [`meow-ascii cats` on Repology](https://repology.org/project/meow-ascii-cats).
TL;DR current status:

- [x] NixOS
- [x] Debian Stable
- [x] Ubuntu
- [x] AUR
- [ ] Fedora
- [ ] EPEL
- [x] Homebrew
- [ ] WinGet
- [ ] Extras
  - [ ] Gentoo
  - [ ] Alpine
  - [ ] OpenBSD
  - [ ] FreeBSD
  - [ ] openSUSE

## Installation

### Ubuntu / Debian / etc

Just run:
```sh
sudo apt update
sudo apt install meow
```

### Nix / Nixos

Try the package out by running `nix-shell -p meow`.

If you want to add it to your configuration, you can add the following to your configuration:

```nix
environment.systemPackages = with pkgs; [
  meow
];
```

### Arch Linux / AUR

Just run the following to install from the AUR:
```sh
git clone https://aur.archlinux.org/meow.git
cd meow
makepkg -si
```

### Mac OS / Homebrew

Just run
```sh
brew install pixelsergey/core/meow
```

Note that the Homebrew maintainers didn't let me merge my package into the main repo since it "wasn't popular enough".
I think they just don't like cats.

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

## Credits

- Cats
- The concept of meowing
- [How to exit vim](https://stackoverflow.com/questions/11828270/how-do-i-exit-the-vim-editor)
