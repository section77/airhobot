let

  inherit (import <nixpkgs> {}) fetchFromGitHub;

  # cmd:
  #   nix-prefetch-git https://github.com/mozilla/nixpkgs-mozilla
  #
  # Commit date is 2019-12-30 14:34:10 -0500
  nixpkgs-mozilla = fetchFromGitHub {
    owner = "mozilla";
    repo = "nixpkgs-mozilla";
    rev = "c482bfd3dab1bde9590b03e712d73ced15385be4";
    sha256 = "18sxl0fxhbdnrfmkbmgxwsn12qy8dbv6ccb3imyyqbjqb76j8dpi";
  };

  # cmd:
  #   nix-prefetch-git https://github.com/NixOS/nixpkgs-channels --rev refs/heads/nixos-19.09
  #
  # Commit date is 2020-01-10 09:04:32 +0000
  nixpkgs = fetchFromGitHub {
    owner = "NixOS";
    repo = "nixpkgs-channels";
    rev = "9f453eb97ffe261ff93136757cd08b522fac83b7";
    sha256 = "16wdsazc7g09ibcxlqsa3kblzhbbpdpb6s29llliybw73cp37b9s";
  };

  pkgs = import nixpkgs { overlays = [ (import "${nixpkgs-mozilla}/rust-overlay.nix") ]; };

  rust-channel = pkgs.rustChannelOf { channel = "1.40.0"; };
  rust = rust-channel.rust.override { extensions = [  "rustfmt-preview" "clippy-preview" "rust-analysis" ]; };


  my-opencv3 = pkgs.opencv3.override {
    enableGtk2 = true;
    enableFfmpeg = true;
  };

in pkgs.mkShell rec {

  buildInputs = with pkgs; [
    clang
    my-opencv3
    rust
    pkgconfig openssl
    python3
  ];

  shellHook = ''
     export LIBCLANG_PATH="${pkgs.llvmPackages.clang-unwrapped.lib}/lib"
  '';
}
