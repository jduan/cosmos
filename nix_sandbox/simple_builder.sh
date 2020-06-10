# How to use this builder script in a derivation?
# simple = derivation { name = "simple"; builder = "${bash}/bin/bash"; args = [ ./simple_builder.sh ]; gcc = gcc; coreutils = coreutils; src = ./simple.c; system = builtins.currentSystem; }
export PATH="$coreutils/bin:$gcc/bin"
mkdir $out
gcc --version
gcc -o $out/simple $src
