extern crate rand;

use rand::Rng;

fn main() {
    let mut m: Vec<P> = Vec::with_capacity(1331);
    let dp: (f32, f32, f32) = (-500.0,-500.0,-500.0);
    let dn: (f32, f32, f32) = (500.0,500.0,500.0);

    for n in 0..1331 {
        let _a: u32 = (n % 121) / 11;
        let _b: u32 = (n % 121) % 11;
        let _c: u32 = n / 121;
        let a = dp.0 + (dn.0 - dp.0) * (_a as f32)/10.0;
        let b = dp.1 + (dn.1 - dp.1) * (_b as f32)/10.0;
        let c = dp.2 + (dn.2 - dp.2) * (_c as f32)/10.0;

        let p = P {_x: _a, _y: _b, _z: _c, x: a, y: b, z: c, c1: (0,0,0,0) };

        m.push(p);
        println!("ind: {} _x: {} _y: {} z: {} x: {} y: {} z: {}",
            n, _a, _b, _c, a, b, c);
    }

    let mut t = [0.0;200];

    for n in 0..20 {
        t[n] = (n as f32)*0.1*3.14159;
    }

    for p in t.iter_mut() {
        let counts = rand::thread_rng().gen_range(0,256);
        let pt: (f32, f32) = orbit(100.0,0.33,*p);
        println!("x: {} y: {} count: {}", pt.0, pt.1, counts);
        let pos: (i32, i32, i32) = fnn3d();
    }
}

struct P {
    _x: u32,
    _y: u32,
    _z: u32,
    x: f32,
    y: f32,
    z: f32,
    c1: (u64, u64, u64, u64),
}

fn orbit(c: f32, e: f32, phi: f32) -> (f32, f32) {
    let x: f32 = c*phi.cos() / ( 1.0 + e * phi.cos() );
    let y: f32 = c*phi.sin() / ( 1.0 + e * phi.cos() );
    return (x,y);
}
// doesn't work
fn fnn3d(x: f32, y: f32, z: f32, xr: &[f32],
    yr: &[f32], zr: &[f32]) -> (i32, i32, i32) {

    let c: (f32, f32, f32) = (x,y,z);
    for i in 1..3 {

    }
    return (ix, iy, iz);
}
// probably doesn't work
fn fnn(x: f32, a: &[f32]) -> i32 {
    if a.len() > 1 {
        let l: i32 = a.len();
        let m: i32 = l/2;
        if a[m] >= x {
            let pos = fnn(x, a[0..m]);
        }
        else {
            let pos = fnn(x, a[m..l-1]) + m;
        }
        return pos;
    }
    else {
        return 0;
    }
}
