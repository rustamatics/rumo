pub fn lnbreak() {
    // Provide a visual line break for development mode
    #[cfg(debug_assertions)]
    print!("\n\n\n");
}
