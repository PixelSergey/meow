pub fn print_cats(literally: bool, count: u16) {
    if literally {
        println!("I am LITERALLY this cat:");
    }

    for _ in 0..count {
        println!("meow");
    }
}
