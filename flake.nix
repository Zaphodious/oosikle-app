{
  description = "Oosikle";
  inputs = {

    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      utils,
      rust-overlay,
      naersk,
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rust = pkgs.rust-bin.stable.latest.default;
        naersk-lib = naersk.lib."${system}".override {
          cargo = rust;
          rustc = rust;
        };
        libPath =
          with pkgs;
          lib.makeLibraryPath [
            fontconfig
            libxkbcommon
            wayland
            libGL
          ];
      in
      {
        defaultPackage = naersk-lib.buildPackage ./.;
        devShell =
          with pkgs;
          mkShell {
            nativeBuildInputs = with pkgs; [
              pkg-config
              gobject-introspection
              cargo
              cargo-tauri
              bun
              rust
              rust-analyzer
              rustup
              
            ];
            buildInputs = with pkgs; [
              at-spi2-atk
              atkmm
              cairo
              gdk-pixbuf
              glib
              gtk3
              harfbuzz
              librsvg
              libsoup_3
              pango
              webkitgtk_4_1
              openssl
              sqlitebrowser
              graphviz

              eza
              fd
              pre-commit
              live-server
              dia
              

              #egui
              trunk
              pkg-config
              libxkbcommon
              libGL
              fontconfig
              wayland
              xorg.libXcursor
              xorg.libXrandr
              xorg.libXi
              xorg.libX11
            ];

            #RUST_SRC_PATH = rust;

            #LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
            LD_LIBRARY_PATH = "$LD_LIBRARY_PATH:${libPath}";
            RUST_SRC_PATH = "${rustPlatform.rustLibSrc}";

            #Try to make Slint/Winit start in x11 mode (for file drag and drop)
            WINIT_UNIX_BACKEND = "x11";
            DISPLAY=":0";
            WAYLAND_DISPLAY="";

            shellHook = ''
              export PATH=$PATH:~/.cargo/bin;
              export XDG_DATA_DIRS=${gsettings-desktop-schemas}/share/gsettings-schemas/${gsettings-desktop-schemas.name}:${gtk3}/share/gsettings-schemas/${gtk3.name}:$XDG_DATA_DIRS;
              export GIO_MODULE_DIR="${pkgs.glib-networking}/lib/gio/modules/";
                      alias ls=eza
                      alias find=fd
            '';

          };
      }
    );
}
