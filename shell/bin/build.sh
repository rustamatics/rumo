#!/bin/bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
ROOT="$( cd $DIR/../ && pwd)"

external_build=$ROOT/app/.externalNativeBuild
cd $ROOT

if [ -d $external_build ]; then
  echo "Clearing External Build Cache"
  rm -rf $external_build;
else
  echo "externalNativeBuild: $external_build is clean"
fi

# $ROOT/gradlew clean && \
RUST_APP_NAME=hello \
RUST_APP_ROOT=$ROOT/../examples/hello \
$ROOT/gradlew build $@

