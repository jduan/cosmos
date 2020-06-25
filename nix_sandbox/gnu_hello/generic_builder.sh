set -e
# First unset PATH, because it's initially set to a non-existant path.
unset PATH
for p in $buildInputs; do
  export PATH=$p/bin/${PATH:+:}$PATH
done

tar -xf $src

# Find a directory where the source has been unpacked and cd into it
for d in *; do
  if [ -d "$d" ]; then
    cd "$d"
    break
  fi
done

./configure --prefix=$out
make
make install
