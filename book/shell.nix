#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

let

  nixpkgsPath =
    let
      rev = "368c3a52976f7848bebecd1d79bcdbf212bae189"; # branch release-25.05
    in
      builtins.fetchTarball {
        url = "https://github.com/NixOS/nixpkgs/archive/${rev}.tar.gz";
        sha256 = "sha256:12cyfh0w84lqwvcy5zkk6r2x22anjhkylg875w4m5y0difk7sc2d";
      };

  pkgs = import nixpkgsPath {};

  inherit (pkgs) lib;

in {
  inherit pkgs;

  shell = with pkgs; mkShell {
    nativeBuildInputs = [
      pkg-config
      openssl
      rustup
      html5validator
      linkchecker
    ] ++ lib.optionals hostPlatform.isDarwin [
      libiconv
      darwin.apple_sdk.frameworks.Security
    ];
  };
}
