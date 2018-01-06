use std::io::{Bytes, Read};
use std::fmt;

pub struct VirtualMachine {
    memory: [u16; 1 << 15],
    registers: [u16; 8],
    stack: Vec<u16>,
}

impl VirtualMachine {
    pub fn init<R: Read>(&mut self, buffer: Bytes<R>) {
        let mut drop = false;
        let mut pos = 0;
        for byte in buffer {
            if drop {
                self.memory[pos] += (byte.expect("error reading file") as u16) << 8;
                pos += 1;
                drop = false;
            } else {
                self.memory[pos] = byte.expect("error reading file") as u16;
                drop = true;
            }
        }
    }

    pub fn new() -> VirtualMachine {
        VirtualMachine {
            memory: [0; 1 << 15],
            registers: [0; 8],
            stack: vec![],
        }
    }

    fn exec(&mut self, pos: usize) -> Option<usize> {
        match self.memory[pos] {
            0 => None,
            1 => {
                self.registers[(self.memory[pos + 1] % (1 << 15)) as usize] = self.memory[pos + 2];
                Some(pos + 3)
            }
            2 => None,
            3 => None,
            4 => {
                self.registers[(self.memory[pos + 1] % (1 << 15)) as usize] =
                    if self.memory[pos + 2] == self.memory[pos + 2] {
                        1
                    } else {
                        0
                    };
                Some(pos + 4)
            }
            5 => {
                self.registers[(self.memory[pos + 1] % (1 << 15)) as usize] =
                    if self.memory[pos + 2] >= self.memory[pos + 2] {
                        1
                    } else {
                        0
                    };
                Some(pos + 4)
            }
            6 => None,
            7 => None,
            8 => None,
            9 => None,
            10 => None,
            11 => None,
            12 => None,
            13 => None,
            14 => None,
            15 => None,
            16 => None,
            17 => None,
            18 => None,
            19 => None,
            20 => None,
            _ => None,
        }
    }

    pub fn run(&mut self) {
        let mut pos = 0;
        loop {
            match self.exec(pos) {
                Some(n) => pos = n,
                None => break,
            }
        }
    }
}

impl fmt::Debug for VirtualMachine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Memory (10 first numbers): {:?}
Registers: {:?}
Stack (top element): {:?}",
            &self.memory[0..10],
            self.registers,
            self.stack.last()
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
            res.push((x % (1 << 8)) as u8);
            res.push((x >> 8) as u8);
        }
        res
    }
    // Simple example from the spec:  9,32768,32769,4,19,32768

    #[test]
    fn init_vm() {
        let simple_example: Cursor<Vec<u8>> =
            Cursor::new(to_binary(&[9, 32768, 32769, 4, 19, 32768]));
        let mut vm_simple_example = VirtualMachine::new();
        vm_simple_example.init(simple_example.bytes());
        assert_eq!(vm_simple_example.memory[1], 32768)
    }

    #[test]
    fn setters_op() {
        let mut vm = VirtualMachine::new();
        for (i, &x) in [1, 32768, 42, 4, 32769, 1, 1, 5, 32770, 2, 48, 25]
            .iter()
            .enumerate()
        {
            vm.memory[i] = x;
        }
        vm.run();
        assert_eq!(vm.registers[0], 42);
        assert_eq!(vm.registers[1], 1);
        assert_eq!(vm.registers[2], 1);
    }
}
