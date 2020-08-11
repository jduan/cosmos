# This is a function that takes two parameters: pkgs and attrs
pkgs: attrs:
with pkgs;
let defaultArgs = {
  builder = "${bash}/bin/bash";
  args = [ ./generic_builder.sh ];
  baseInputs = [gnutar gzip gnumake coreutils gawk gnused gnugrep clang.bintools.bintools_bin clang];
  buildInputs = [];
  system = builtins.currentSystem;
};
in
  # Merge the additional "attrs" with the default args
  derivation (defaultArgs // attrs)
