cmake_minimum_required(VERSION 3.4.1)

add_library( # Sets the name of the library.
             mini
             SHARED

             # Provides a relative path to your source file(s).
             src/mini.cpp )

set_target_properties(
  mini PROPERTIES CXX_STANDARD 11 CXX_STANDARD_REQUIRED ON)
