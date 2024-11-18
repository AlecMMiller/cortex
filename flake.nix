{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs =
    inputs@{ nixpkgs, flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = nixpkgs.lib.systems.flakeExposed;
      perSystem =
        {
          lib,
          pkgs,
          system,
          config,
          ...
        }:
        let
          libraries = with pkgs; [
            webkitgtk_4_1
            gtk3
            openssl
          ];
        in
        {
          devShells.default = pkgs.mkShell {

            buildInputs = with pkgs; [
              libiconv

              # Rust
              cargo
              rustc
              rustfmt

              # Node.js
              nodejs # feel free to change the version
              yarn

              openssl

              gtk3
              libsoup_3
              webkitgtk_4_1

              pkg-config
            ];

            shellHook = ''
              export PATH=~/.cargo/bin/:$PATH
              export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath libraries}:$LD_LIBRARY_PATH
              export XDG_DATA_DIRS=${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS
            '';
          };
        };
    };
}
