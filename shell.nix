# you may be asking: why have this if you already have flake.nix?
# well, to test building seabun in "pure" mode (no inherited PATH),
# as `nix develop` doesn't support it
# 
# to do that run `nix-shell --pure shell.nix`

{ pkgs ? import <nixpkgs> {} }:

let
  # user-friendly shell; replace it with your preferred one, maybe
  useShell = {
    pkg  = pkgs.fish;
    cmd = "fish";
  };
  
  # text editor; replace it with your preferred one, maybe
  useEditor = pkgs.micro;
in
pkgs.mkShell {
  name = "seabun-dev-shell";
  
  packages = with pkgs; [
    #qemu # for testing other architectures
    
    lld # llvm linker
    libllvm # necessary for the llvm-sys crate
    # * i'll try to update the language when new versions drop;
    # else i'll leave it at version 21.1 (if llvmPackages_21 drops)
    
    # rust components
    cargo
    rustc
    rustfmt
    rust-analyzer
    clippy
  ] ++ [
    useShell.pkg
    useEditor
  ];
  
  shellHook = ''
    exec "${useShell.cmd}"
  '';
}
