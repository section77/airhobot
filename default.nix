let

  inherit (import <nixpkgs> {}) fetchFromGitHub;

  # cmd:
  #   nix-prefetch-git https://github.com/NixOS/nixpkgs-channels
  #
  # 2020-02-09T08:22:00+01:00
  pkgs = import (fetchFromGitHub {
    owner = "NixOS";
    repo = "nixpkgs-channels";
    rev = "8130f3c1c2bb0e533b5e150c39911d6e61dcecc2";
    sha256 = "154nrhmm3dk5kmga2w5f7a2l6j79dvizrg4wzbrcwlbvdvapdgkb";
  }) {};

  my-opencv4 = pkgs.opencv4.override {
    enableGtk3 = true;
    enableFfmpeg = true;
  };

in pkgs.mkShell rec {

  buildInputs = with pkgs; [
    clang
    my-opencv4
    ffmpeg
    rustc cargo rls rustfmt
    pkgconfig openssl
    python3
    gnome3.gtk glib cairo pango atk gdk_pixbuf

  ];


  shellHook = ''
     export LIBCLANG_PATH="${pkgs.llvmPackages.clang-unwrapped.lib}/lib"
  '';
}
