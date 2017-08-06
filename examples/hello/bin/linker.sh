#!/bin/bash
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

TOOLS=$ANDROID_SDK/build-tools/25.0.0
LINKER=$TOOLS/x86_64-linux-android-ld
GCC=/home/bailey/Android/standalone-arm/bin/arm-linux-androideabi-gcc

echo "$GCC $@" > $DIR/../.linker.log
$GCC $@
