fn main() {
    let mut x = 0;
    loop {
        println!("y");
        x = x + 1;
        if x > 15_000_000 {
            break;
        }
    }
}
