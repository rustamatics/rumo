# Kinito

Cross-platform Mobile build tool for Rust based Apps — Supporting Android / iOS

## Installing

    $ cargo install kinito

## Usage

    # Compile your app into a APK
    $ kinito build

    # Load the appropriate APK onto your emulator / device
    $ kinito device-install

## Android Overview

Kinito let's you build native Android apps with Rust while not having to write any Java.

The entire build process is wrapped up into a simple command.
This is achieved by embedding your rust app as a shared library within a **NativeActivity**.

A shell project is provided automatically, with your own app injected inside, then built using Gradle to produce runnable APKs.

## Getting Started (Android)

#### 1. Download Android NDK

  - Create standalone toolchains for each cpu arch you wish to build for.

      *(ie. arm, x86, mips)*

  - Use **$ANDROID_NDK/build/tools/make_standalone_toolchain.py** to create the standalone toolchains.

#### 2. Setup your Cargo.toml
  - Add a dylib section in your Cargo.toml

        [lib]
        crate-type = ["dylib"]


   - Place a .cargo/config to specify the linkers for each CPU ABI you wish to compile for.


        [target.x86_64-linux-android]
        linker = "/android/standalone-x86_64/bin/x86_64-linux-android-gcc"

        [target.arm-linux-androideabi]
        linker = "/android/standalone-arm/bin/arm-linux-androideabi-gcc"

  - Setup your **android_main()** entry point inside your app code. *(ie. Setup OpenGL context)*

 See the examples for more details.


#### 3. You can now run **kinito build** which will produce Android APKs *(arm-linux-androideabi)* in your ./project/target directory.

  - You can now achieve fast iterative development on the Desktop (x86) while building for mobile at the same time.

  - Use cross platform toolkits (ie. glutin) which supports Android/iOS/PC
