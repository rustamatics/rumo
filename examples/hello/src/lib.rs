extern crate android_ffi;
extern crate kinito;

use std::boxed::Box;

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
pub extern "C" fn rust_android_main(app: *mut ()) {
    android_ffi::android_main2(app as *mut _, move |c, v| unsafe { main(c, v) });
}

struct EventHandler {}

impl android_ffi::SyncEventHandler for EventHandler {
    fn handle(&mut self, event: &android_ffi::Event) {
        android_ffi::write_log(&format!("handling event: #{:?}", event)[..]);
    }
}

fn main(_: isize, _: *const *const u8) {
    android_ffi::write_log("hello::main has been called!");
    let handler = Box::from(EventHandler{});

    android_ffi::add_sync_event_handler(handler);
}
