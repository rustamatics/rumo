// #![no_std]
extern crate android_ffi;

#[no_mangle]
pub extern "C" fn entry_point()  -> i32 {
    println!("hello from entry_point: {}%", 100);
    33
}

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn hello_android_main(app: *mut ()) {
    android_ffi::android_main2(app as *mut _, move |c, v| unsafe { main(c, v) });
}

fn main(_: isize, _: *const *const u8) {
    android_ffi::write_log("Main has been called!");
}
