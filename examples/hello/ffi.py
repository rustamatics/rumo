
try:
    from cffi import FFI
except ImportError:
    print "pip install cffi, included with PyPy"

ffi = FFI()
lib = ffi.dlopen("target/x86_64-unknown-linux-gnu/release/libhello.so")
lib2 = ffi.dlopen("../minic/build/libmini.so")

#  print lib
# <cffi.api.FFILibrary_./libtreble.dylib object at 0x107f440d0>

ffi.cdef('int entry_point(void);')
ffi.cdef('int mini_entry(void);')

print "attempting entry_point()", lib.entry_point()
print "attempting mini_entry()", lib2.mini_entry()

