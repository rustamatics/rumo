#[cfg(target_os = "android")]
extern crate android_ffi;
// extern crate rumo;

use std::boxed::Box;

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
#[cfg(target_os = "android")]
pub extern "C" fn rust_android_main(app: *mut ()) {
    android_ffi::android_main2(app as *mut _, move |c, v| unsafe { main(c, v) });
}

struct EventHandler {}

#[cfg(target_os = "android")]
impl android_ffi::SyncEventHandler for EventHandler {
    fn handle(&mut self, event: &android_ffi::Event) {
        android_ffi::write_log(&format!("handling event: #{:?}", event)[..]);
    }
}

#[cfg(target_os = "android")]
fn main(_: isize, _: *const *const u8) {
    let handler = Box::from(EventHandler{});
    android_ffi::write_log("hello::main has been called!");
    android_ffi::add_sync_event_handler(handler);
}
