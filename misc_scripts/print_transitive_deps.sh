#!/usr/bin/env bash
# Given a maven coordinate, this script prints its transitive dependencies.
#
# Example:
# print_transitive_deps.sh org.eclipse.jetty.http2 http2-http-client-transport 9.4.11.v20180605

if [ "$#" -ne 3 ]; then
  echo "Usage: $0 <groupId> <artifactId> <version>"
  exit
fi

POM_DIR="$(echo "$1" | tr . /)/$2/$3"
POM_PATH="$POM_DIR/$2-$3.pom"

mkdir -p "$HOME/.m2/repository/$POM_DIR"
wget -q -O "$HOME/.m2/repository/$POM_PATH" "https://repo.maven.apache.org/maven2/$POM_PATH"
mvn -f "$HOME/.m2/repository/$POM_PATH" dependency:tree
