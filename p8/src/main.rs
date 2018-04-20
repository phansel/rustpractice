// warning: this doesn't work
extern crate rand;
extern crate websocket;

use rand::Rng;
use std::io::prelude::*;
use std::net::TcpStream;

const ip_addr: &str = "127.0.0.1:34254";

fn main() {
    let mut stream = pub struct TcpStream::connect(ip_addr).unwrap();

    let _ = stream.write(&[1]);
    let _ = stream.read(&mut [0; 128]);
}
