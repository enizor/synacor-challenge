use std::io::{Read};
use std::fs::File;
use std::env::args;

mod vm;
use vm::VirtualMachine;

fn main() {

    let  input = args()
        .nth(1).expect(
        "Please enter the path for the binary to read as the only argument",
    );
    let f = File::open(input.trim()).unwrap();
    let mut vm = VirtualMachine::new();
    vm.init(f.bytes());
}
