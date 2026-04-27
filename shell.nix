{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  name = "seabun-dev";
  packages = with pkgs; [
    micro # text editor; replace it with your preferred one
    
    libllvm # necessary for the llvm-sys crate
    lld # llvm linker
    #quickemu # for testing other architectures
    
    # rust components
    cargo
    rustc
    rustfmt
    rust-analyzer
    clippy
  ];
}
