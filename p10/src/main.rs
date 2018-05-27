extern crate rand;

use rand::Rng;

fn main() {
    let mut l: Vec<L> = Vec::with_capacity(19);
    for i in 0..18 {
        let _r = n_to_char(rand::thread_rng().gen_range(1,6));
        let _rds = ('0','0','0','0','0','0');
        let _sts = ('0','0','0','0','0','0');
        let _l = L { x: i, r: _r, rds: _rds, sts: _sts};
        l.push(_l);
        println!("r is {}", _r);
    }
}

struct L {
    x: i32,
    r: char,
    rds: (char, char, char, char, char, char),
    sts: (char, char, char, char, char, char),
}

fn char_to_n (c: char) -> i32 {
    match c {
        'w' => 1,
        'r' => 2,
        'g' => 3,
        'b' => 4,
        's' => 5,
        _ => {println!("Error: what is {}?",c);0},
    }
}

fn n_to_char (n: i32) -> char {
    match n {
        1 => 'w',
        2 => 'r',
        3 => 'g',
        4 => 'b',
        5 => 's',
        _ => {println!("Error: what is {}?",n);'u'},
    }
}
