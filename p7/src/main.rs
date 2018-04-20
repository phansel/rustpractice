fn main() {
    // EXAMPLE OF OVERFLOW
    let mut x: i8 = 0;
    loop {
        println!("x is {}", x);
        x = x + 1;
    }
}
