with (import <nixpkgs> {});
derivation {
  name = "simple";
  builder = "${bash}/bin/bash";
  args = [ ./simple_builder.sh ];
  # "inherit foo" is equivalent to "foo = foo"
  # "inherit foo bar" is equivalent to "foo = foo; bar = bar;"
  inherit gcc coreutils;
  src = ./simple.c;
  system = builtins.currentSystem;
}
