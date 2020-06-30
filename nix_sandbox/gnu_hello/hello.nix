let
  pkgs = import <nixpkgs> {};
  # import the function defined in "autotools.nix" and call it with "pkgs"
  mkDerivation = import ./autotools.nix pkgs;
in mkDerivation {
  name = "hello";
  # Note that you can't define src as a string "./hello-2.10.tar.gz"!
  src = ./hello-2.10.tar.gz;
}
