#! /usr/bin/env nix-shell
#! nix-shell -i bash --pure
#! nix-shell -p bash nix

# for convenience, for people who don't have nix-command and flakes enabled system-wide
nix --extra-experimental-features 'nix-command flakes' develop #.

