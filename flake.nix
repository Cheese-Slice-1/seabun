{
  description = "Seabun devshell";
  
  inputs = {
  	nixpkgs.url = "github:nixos/nixpkgs/nixos-25.11";
  };
  
  outputs =
  	{ nixpkgs, ... }:
  	let
  	  system = "x86_64-linux";
  	  pkgs = import nixpkgs { inherit system; };
    in
    {
  	  devShells.${system}.simple = pkgs.mkShell {
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
  	  };
    };
}
