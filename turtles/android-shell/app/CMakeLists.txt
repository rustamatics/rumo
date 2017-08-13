# For more information about using CMake with Android Studio, read the
# documentation: https://d.android.com/studio/projects/add-native-code.html

# Sets the minimum version of CMake required to build the native library.

cmake_minimum_required(VERSION 3.4.1)

# Creates and names a library, sets it as either STATIC
# or SHARED, and provides the relative paths to its source code.
# You can define multiple libraries, and CMake builds them for you.
# Gradle automatically packages shared libraries with your APK.

add_library( # Sets the name of the library.
             native-lib

             # Sets the library as a shared library.
             SHARED

             # Provides a relative path to your source file(s).
             src/main/cpp/native-lib.cpp )

# Define RUST_APP_ROOT for target cross compilation.
# Value pulled from Environment with a fallback
if (DEFINED ENV{RUST_APP_ROOT})
  set( RUST_APP_ROOT "$ENV{RUST_APP_ROOT}" )
else()
  set( RUST_APP_ROOT "${CMAKE_BINARY_DIR}/../../../../src/main/rust" )
endif()

if (DEFINED ENV{RUST_APP_NAME})
  set( RUST_APP_NAME "$ENV{RUST_APP_NAME}" )
else()
  set( RUST_APP_NAME rust_app )
endif()

if (ANDROID_ABI STREQUAL "x86_64")
  set(RUST_ARCH x86_64-linux-android)
elseif(ANDROID_ABI STREQUAL "x86")
  set(RUST_ARCH i686-linux-android)
elseif(ANDROID_ABI STREQUAL "armeabi-v7a")
  set(RUST_ARCH armv7-linux-androideabi)
elseif(ANDROID_ABI STREQUAL "armeabi")
  set(RUST_ARCH arm-linux-androideabi)
elseif(ANDROID_ABI STREQUAL "mips")
  set(RUST_ARCH mips-unknown-linux-gnu)
else()
  # MESSAGE ( FATAL_ERROR "ANDROID_ABI not supported: ${ANDROID_ABI}" )
  set(RUST_ARCH x86_64-unknown-linux-gnu)
endif()

set(RUST_APP_LIB
  "${RUST_APP_ROOT}/target/${RUST_ARCH}/release/lib${RUST_APP_NAME}.so")

if( NOT EXISTS ${RUST_APP_LIB} )
  MESSAGE(FATAL_ERROR "RUST_APP_LIB:${RUST_APP_LIB} not found")
endif()

set(${CMAKE_C_FLAGS}, "${CMAKE_C_FLAGS}")
add_library(native_app_glue STATIC
      ${ANDROID_NDK}/sources/android/native_app_glue/android_native_app_glue.c)

# now build app's shared lib
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=gnu++11 -Wall -Werror")
set(CMAKE_SHARED_LINKER_FLAGS "${CMAKE_SHARED_LINKER_FLAGS} -u ANativeActivity_onCreate")

target_include_directories(native-lib PRIVATE
      ${ANDROID_NDK}/sources/android/native_app_glue)

# Specifies libraries CMake should link to your target library. You
# can link multiple libraries, such as libraries you define in this
# build script, prebuilt third-party libraries, or system libraries.

target_link_libraries(native-lib
  android
  native_app_glue
  EGL
  GLESv1_CM

  # Rust target links
  ${RUST_APP_LIB}

  # Links the target library to the log library
  # included in the NDK
  # ${log-lib}
  log
  )


set_target_properties(
  native-lib PROPERTIES CXX_STANDARD 11 CXX_STANDARD_REQUIRED ON)

# MESSAGE ( STATUS "ANDROID_ABI: "   ${ANDROID_ABI} )
# MESSAGE ( STATUS "RUST_ARCH: "     ${RUST_ARCH} )
# MESSAGE ( STATUS "RUST_APP_ROOT: " ${RUST_APP_ROOT} )
# MESSAGE ( STATUS "RUST_APP_NAME: " ${RUST_APP_NAME} )
# MESSAGE ( STATUS "RUST_APP_LIB: "  ${RUST_APP_LIB} )

