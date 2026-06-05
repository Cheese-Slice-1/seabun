#! /usr/bin/env nix-shell
#! nix-shell -i bash -p bash nix --pure

# for convenience, for people who don't have nix-command and flakes enabled system-wide
SEABUN_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
echo "$SEABUN"

nix --extra-experimental-features 'nix-command flakes' develop $SEABUN_DIR

