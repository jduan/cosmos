# How to use this builder to build a package?
# fire up "nix repl"
# d = derivation { name = "foo"; builder = "${bash}/bin/bash"; args = [./builder.sh]; system = builtins.currentSystem;  }
# :l <nixpkgs>
# :b d

# List exported variables
declare -xp
echo foo > $out
