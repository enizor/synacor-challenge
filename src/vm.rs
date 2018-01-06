use std::io::{Bytes, Read};
use std::fmt;

pub struct VirtualMachine {
    memory: [u16; 1 << 15],
    registers: [u16; 8],
    stack: Vec<u16>,
    exec_pointer: u16,
}

impl VirtualMachine {
    pub fn new<R: Read>(buffer: Bytes<R>) -> VirtualMachine {
        let mut vm = VirtualMachine {
            memory: [0; 1 << 15],
            registers: [0; 8],
            stack: vec![],
            exec_pointer: 0,
        };
        let mut drop = false;
        let mut pos = 0;
        for byte in buffer {
            println!("{:?}", byte);
            if drop {
                vm.memory[pos] += (byte.expect("error reading file") as u16) << 8;
                pos += 1;
                drop = false;
            } else {
                vm.memory[pos] = byte.expect("error reading file") as u16;
                drop = true;
            }
        }
        vm
    }
}

impl fmt::Debug for VirtualMachine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Memory (10 first numbers): {:?}
Registers: {:?}
Stack (top element): {:?},
Execution Pointer: {:?}",
            &self.memory[0..10],
            self.registers,
            self.stack.last(),
            self.exec_pointer
        )
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use super::*;
    fn to_binary(input: &[u16]) -> Vec<u8> {
        let mut res = vec![];
        for &x in input {
            println!("{} {} {}", x, x >> 8, x % (1 << 8));
            res.push((x % (1 << 8)) as u8);
            res.push((x >> 8) as u8);
        }
        println!("{:?}", res);
        res
    }
    // Simple example from the spec:  9,32768,32769,4,19,32768

    #[test]
    fn init_vm() {
        let simple_example: Cursor<Vec<u8>> =
            Cursor::new(to_binary(&[9, 32768, 32769, 4, 19, 32768]));
        let vm_simple_example = VirtualMachine::new(simple_example.bytes());
        println!("{:?}", vm_simple_example);
        assert_eq!(vm_simple_example.memory[1], 32768)
    }

}
