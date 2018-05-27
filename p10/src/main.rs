fn main() {
    let mut l: Vec<L> = Vec::with_capacity(19);
    for i in 0..18 {
        let _a = 'a';
        let _rs = ('0','0','0','0','0','0');
        let _ss = ('0','0','0','0','0','0');
        let _l = L { x: i, a: _a, rs: _rs, ss: _ss};
        l.push(_l);
    }
}

struct L {
    x: i32,
    a: char,
    rs: (char, char, char, char, char, char),
    ss: (char, char, char, char, char, char),
}

fn char_to_n (c: char) -> i32 {
    match c {
        'w' => 1,
        'r' => 2,
        'g' => 3,
        'b' => 4,
        's' => 5,
        _ => {println!("Error: what is {}?",c); 0},
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
