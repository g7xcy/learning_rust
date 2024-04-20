# let
#   nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-23.11";
#   pkgs = import nixpkgs { config = {}; overlays = []; };
# in
#   pkgs.mkShell {
#     nativeBuildInputs = with pkgs.buildPackages; [
#       cargo
#       rustc
#       rust-analyzer
#     ];
    
#     COLORTERM = "truecolor";
#     fish_charset = "utf-8";

#     shellHook = ''
#       fish
#     '';
#   }
{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    gcc
    rustc
    cargo
    rust-analyzer
    ];
  shellHook = ''
    # echo "use nix" > .envrc
  '';
}

