fn main() {
    let mut m: Vec<P> = Vec::with_capacity(100);
    let dp: (f32, f32) = (0.0,0.0);
    let dn: (f32, f32) = (500.0,500.0);

    for n in 0..100 {
        let _a: u32 = n / 10;
        let _b: u32 = n % 10;
        let a = (dn.0 - dp.0) * (_a as f32)/10.0;
        let b = (dn.1 - dp.1) * (_b as f32)/10.0;
        let p = P {_x: _a, _y: _b, x: a, y:b, c1: (0,0,0,0) };

        m.push(p);
        println!("ind: {} _x: {} _y: {} x: {} y: {}",
            n, _a, _b, a, b);
    }
}

struct P {
    _x: u32,
    _y: u32,
    x: f32,
    y: f32,
    c1: (u64, u64, u64, u64),
}
