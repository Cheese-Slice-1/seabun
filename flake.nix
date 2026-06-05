# ez pz method for the people who don't write the file
# just run `nix develop` inside the "seabun" folder and ignore the horrors of tis file

{
  description = "Seabun development shell/environment; everything included ( + micro)";
  
  inputs.nixpkgs = {
    url = "https://github.com/NixOS/nixpkgs/archive/549bd84d6279f9852cae6225e372cc67fb91a4c1.tar.gz";
    #sha256 = "sha256-hGdgeU2Nk87RAuZyYjyDjFL6LK7dAZN5RE9+hrDTkDU=";
  };
  
  outputs =
    inputs@{ self, nixpkgs, ... }:
    let
      system = "x86_64-linux"; # idk how to add more, if you're not on x86_64 use `nix-shell`
      pkgs = import  nixpkgs { inherit system; };
      
      # text editor; replace it with your preferred one, maybe
      useEditor = pkgs.micro;
      llvm21 = pkgs.llvmPackages_21;
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        name = "seabun-dev-flake";
        
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
      };
    };
}
