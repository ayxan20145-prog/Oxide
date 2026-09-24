{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  packages = with pkgs; [
    cmake
    pkg-config

    libX11
    libXext
    libXcursor
    libXi
    libXrandr
    libXfixes
    libXrender
    libXtst
    libXScrnSaver
    libxcb
  ];

  shellHook = ''
    export CMAKE_PREFIX_PATH="${
      pkgs.lib.makeSearchPathOutput "dev" "lib/cmake" [
        pkgs.libX11
        pkgs.libXext
        pkgs.libXcursor
        pkgs.libXi
        pkgs.libXrandr
        pkgs.libXfixes
        pkgs.libXrender
        pkgs.libXtst
        pkgs.libXScrnSaver
        pkgs.libxcb
      ]
    }:$CMAKE_PREFIX_PATH"

    export PKG_CONFIG_PATH="${
      pkgs.lib.makeSearchPathOutput "dev" "lib/pkgconfig" [
        pkgs.libX11
        pkgs.libXext
        pkgs.libXcursor
        pkgs.libXi
        pkgs.libXrandr
        pkgs.libXfixes
        pkgs.libXrender
        pkgs.libXtst
        pkgs.libXScrnSaver
        pkgs.libxcb
      ]
    }:$PKG_CONFIG_PATH"

    export LD_LIBRARY_PATH="${
      pkgs.lib.makeLibraryPath [
        pkgs.libX11
        pkgs.libXext
        pkgs.libXcursor
        pkgs.libXi
        pkgs.libXrandr
        pkgs.libXfixes
        pkgs.libXrender
        pkgs.libXtst
        pkgs.libXScrnSaver
        pkgs.libxcb
      ]
    }:$LD_LIBRARY_PATH"
  '';
}
