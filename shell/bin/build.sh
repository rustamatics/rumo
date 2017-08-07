#!/bin/bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
ROOT="$( cd $DIR/../ && pwd)"

external_build=$ROOT/app/.externalNativeBuild

if [ -d $external_build ]; then
  echo "Clearing External Build Cache"
  rm -rf $external_build;
else
  echo "externalNativeBuild: $external_build is clean"
fi

# $ROOT/gradlew clean && \
if [[ "$RUST_APP_NAME" == "" ]]; then
  RUST_APP_NAME=hello
fi

if [[ "$RUST_APP_ROOT" == "" ]]; then
  RUST_APP_ROOT=$(cd $ROOT/../examples/hello && pwd)
fi

echo "RUST_APP_NAME: $RUST_APP_NAME"
echo "RUST_APP_ROOT: $RUST_APP_ROOT"

cd $ROOT
RUST_APP_NAME=$RUST_APP_NAME \
RUST_APP_ROOT=$RUST_APP_ROOT \
  $ROOT/gradlew build $@

