// #![no_std]

#[no_mangle]
pub extern "C" fn entry_point()  -> i32 {
    // print!("hello!");
    println!("hello from entry_point: {}%", 100);
    33
}
