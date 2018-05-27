fn main() {
    let mut l: Vec<L> = Vec::with_capacity(19);
    for i in 0..18 {
        let _rs = ('0','0','0','0','0','0');
        let _ss = ('0','0','0','0','0','0');
        let _l = L { x: i, a:'a', rs: _rs, ss: _ss};
        l.push(_l);
    }
}

struct L {
    x: i32,
    a: char,
    rs: (char, char, char, char, char, char),
    ss: (char, char, char, char, char, char),
}
