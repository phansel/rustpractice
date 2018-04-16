fn main() {
    let _t = true;
    let _f: bool = false;

    let _p = '💩';

    let t = true;

    match_fn(t);
}

struct _P {
    x: f32,
    y: f32,
    b: bool,
}

fn match_fn(b: bool) {
    match b {
        true => println!("true"),
        false => println!("false"),
    }
}
