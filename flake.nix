{
  description = "Seabun development shell -- everything included (+ fish + micro)";
  
  inputs.nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  
  outputs =
    { self, nixpkgs, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
      
      # user-friendly shell; replace it with your preferred one
      useShell = {
        pkg = pkgs.fish;
        cmd = "fish";
      };
      
      # text editor; replace it with your preferred one
      useEditor = pkgs.micro;
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        name = "seabun-dev";
        
        packages = with pkgs; [
          libllvm # necessary for the llvm-sys crate
          lld # llvm linker
          #quickemu # for testing other architectures(?)
          
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
