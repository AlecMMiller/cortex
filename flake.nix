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
            openssl
            libGL
            libxkbcommon
            wayland
          ];
        in
        {
          devShells.default = pkgs.mkShell {

            nativeBuildInputs = with pkgs; [
              pkg-config
              gobject-introspection
              cargo
              cargo-tauri
              libxkbcommon
              libGL
              # WINIT_UNIX_BACKEND=wayland
              wayland
              vulkan-tools
              vulkan-headers
              vulkan-loader
              vulkan-validation-layers
            ];

            buildInputs = with pkgs; [
              at-spi2-atk
              atkmm
              cairo
              gdk-pixbuf
              harfbuzz
              librsvg
              pango
              libiconv

              # Rust
              cargo
              cargo-nextest
              rustc
              rustfmt
              gtk3
              libsoup_3
              pkg-config
            ];

            LD_LIBRARY_PATH = "${pkgs.libxkbcommon}/lib:${pkgs.libGL}/lib:${pkgs.wayland}/lib";
            RUST_BACKTRACE = 1;

            shellHook = ''
              export PATH=~/.cargo/bin/:$PATH
              export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath libraries}:$LD_LIBRARY_PATH
              export XDG_DATA_DIRS=${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS
            '';
          };
        };
    };
}
