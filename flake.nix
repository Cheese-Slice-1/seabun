# ez pz method for the people who don't write the file
# just run `nix develop` inside the "seabun" folder and ignore the horrors of tis file

{
  description = "Seabun development shell -- everything included (+ fish + micro)";
  
  inputs.nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  
  outputs =
    { self, nixpkgs, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
      
      # user-friendly shell; replace it with your preferred one, maybe
      useShell = {
        pkg = pkgs.fish;
        cmd = "fish";
      };
      
      # text editor; replace it with your preferred one, maybe
      useEditor = pkgs.micro;
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        name = "seabun-dev-flake";
        
        packages = with pkgs; [
          #qemu # for testing other architectures(?)
          
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
          exec ${useShell.cmd}
        '';
      };
    };
}
