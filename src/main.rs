extern {
    fn add_one(x: u32) -> u32;
}

fn main() {
    unsafe {
        println!("{}", add_one(41));
    }
}
