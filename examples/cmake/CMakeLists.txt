cmake_minimum_required(VERSION 3.4.1)

add_executable( # Sets the name of the library.
             cmake-example

             # Provides a relative path to your source file(s).
             src/cmake-example.cpp )

set(RUST_ARCH x86_64-unknown-linux-gnu)

set(RUST_APP_LIB
  "${CMAKE_CURRENT_SOURCE_DIR}/../hello/target/${RUST_ARCH}/release/libhello.so")

set(MINI_LIB
  "${CMAKE_CURRENT_SOURCE_DIR}/../minic/build/libmini.so")

if( NOT EXISTS ${RUST_APP_LIB} )
  MESSAGE(FATAL_ERROR "RUST_APP_LIB:${RUST_APP_LIB} not found")
endif()


target_link_libraries(cmake-example
  ${RUST_APP_LIB}
  ${MINI_LIB}
)
