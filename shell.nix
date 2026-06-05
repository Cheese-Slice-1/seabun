# you may be asking: why have this if you already have flake.nix?
# well, to test building seabun in "pure" mode (no inherited PATH),
# as `nix develop` doesn't support it
# 
# to do that run `nix-shell --pure (/path/to/)shell.nix`
# or simply run `nix-shell --pure` in this project's root folder
# 
# to use this in IMPURE mode, omit the "--pure" flag

let
  nixpkgs = {
    url = "https://github.com/NixOS/nixpkgs/archive/cf8cc1201be8bc71b7cbbbdaf349b22f4f99c7ae.tar.gz";
    sha256 = "sha256-hGdgeU2Nk87RAuZyYjyDjFL6LK7dAZN5RE9+hrDTkDU=";
  };
  system = builtins.currentSystem;
  
  pkgs = import (fetchTarball nixpkgs) { inherit system; };
  
  # text editor; replace it with your preferred one, maybe
  useEditor = pkgs.micro;
  llvm21 = pkgs.llvmPackages_21;
in
pkgs.mkShell {
  name = "seabun-dev-shell";
  
  packages = (with pkgs; [
    #qemu # for testing other architectures
    libtinfo
    
    # rust components
    cargo
    rustc
    rustfmt
    rust-analyzer
    clippy
  ])
  ++ (with llvm21; [
    lld # llvm linker
    libllvm # necessary for the llvm-sys crate
  ])
  ++ [ useEditor ];
  
  shellHook = ''
    echo '* run "cargo clippy -- -A warnings" to see if there are any errors'
    echo '* to get a full diagnostic, omit everything starting from the "--"'
  '';
}
