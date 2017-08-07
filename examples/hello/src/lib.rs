// #![no_std]

#[no_mangle]
pub extern "C" fn entry_point()  -> i32 {
    // print!("hello!");
    format!("hello from entry_point");
    1
}
