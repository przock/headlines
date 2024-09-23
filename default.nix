let
  pkgs = import <nixpkgs> {};
in
pkgs.rustPlatform.buildRustPackage rec {
  pname = "headlines";
  version = "0.2.3";

  src = ./.;

  nativeBuildInputs = with pkgs; [
    pkg-config
    openssl
    openssl.dev
  ];

  cargoLock.lockFile = ./Cargo.lock;
  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
}
