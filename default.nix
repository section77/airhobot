let

  inherit (import <nixpkgs> {}) fetchFromGitHub;

  # cmd:
  #   nix-prefetch-git https://github.com/mozilla/nixpkgs-mozilla
  #
  # Commit date is 2019-01-25 17:40:39 +0100
  nixpkgs-mozilla = fetchFromGitHub {
    owner = "mozilla";
    repo = "nixpkgs-mozilla";
    rev = "507efc7f62427ded829b770a06dd0e30db0a24fe";
    sha256 = "17p1krbs6x6rnz59g46rja56b38gcigri3h3x9ikd34cxw77wgs9";
  };

  # cmd:
  #   nix-prefetch-git https://github.com/NixOS/nixpkgs-channels --rev refs/heads/nixos-18.09
  #
  # Commit date is 2019-01-23 07:13:01 -0500
  nixpkgs = fetchFromGitHub {
    owner = "NixOS";
    repo = "nixpkgs-channels";
    rev = "749a3a0d00b5d4cb3f039ea53e7d5efc23c296a2";
    sha256 = "14dqndpxa4b3d3xnzwknjda21mm3p0zmk8jbljv66viqj5plvgdw";
  };

  pkgs = import nixpkgs { overlays = [ (import "${nixpkgs-mozilla}/rust-overlay.nix") ]; };

  rust-channel = pkgs.rustChannelOf { channel = "1.32.0"; };
  rust = rust-channel.rust.override { extensions = [ "clippy-preview" "rustfmt-preview" ]; };


  my-opencv3 = pkgs.opencv3.override {
    enableGtk2 = true;
  };

in pkgs.mkShell rec {

  buildInputs = with pkgs; [
    clang
    my-opencv3
    rust
  ];

  shellHook = ''
     export LIBCLANG_PATH="${pkgs.llvmPackages.clang-unwrapped.lib}/lib"
  '';
}
