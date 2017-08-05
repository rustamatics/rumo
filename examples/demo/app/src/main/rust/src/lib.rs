#[no_std]


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

#[no_mangle]
pub extern fn execute()  {
    print!("hello!")
}
