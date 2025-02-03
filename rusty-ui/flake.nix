{
  description = "Rusty ui";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs, ... }@inputs:
  let
    system = "x86_64-linux";
    pkgs = import nipkgs { inherit system; };
  in {
    devShells.${system}.default = pkgs.mkShell {
      packages = [
        # here the pkgs that you need (development packages)
        
        pkgs.tailwindcss
        pkgs.cargo
        pkgs.rustup

        # Or you can even load them from another flake
        # inputsFrom = [ inputs.flake-tmux.devShells.${system}.default ];
      ];
      shellHook = ''
        # Here you split with tmux the shell
        # and you start the different servers
      '';
    };
  };
}
