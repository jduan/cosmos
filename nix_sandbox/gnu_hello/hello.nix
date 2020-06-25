with (import <nixpkgs> {});
derivation {
  name = "hello";
  builder = "${bash}/bin/bash";
  args = [ ./generic_builder.sh ];
  buildInputs = [gnutar gzip gnumake coreutils gawk gnused gnugrep clang.bintools.bintools_bin clang];
  src = ./hello-2.10.tar.gz;
  system = builtins.currentSystem;
}
