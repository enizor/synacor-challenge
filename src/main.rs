use std::io::{stdin, Read};
use std::fs::File;
use std::env::args;

mod vm;
use vm::VirtualMachine;

fn main() {

    let mut input = args()
        .nth(1).expect(
        "Please enter the path for the binary to read as the only argument",
    );
    println!("{:?}",input );
    let f = File::open(input.trim()).unwrap();
    let vm = VirtualMachine::new(f.bytes());
    println!("{:?}", vm);
}
