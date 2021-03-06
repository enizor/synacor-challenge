use std::io::{stdin, Bytes, Read};
use std::fmt;

pub struct VirtualMachine {
    memory: [u16; (1 << 15) + 8],
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
            memory: [0; (1 << 15) + 8],
            stack: vec![],
        }
    }

    fn decode(&self, pos: usize) -> u16 {
        match self.memory[pos] {
            0...32767 => self.memory[pos],
            32768...32775 => self.memory[self.memory[pos] as usize],
            _ => panic!("Memory error: int too big"),
        }
    }

    fn exec(&mut self, pos: usize) -> Option<usize> {
        match self.memory[pos] {
            0 => None,
            1 => {
                self.memory[self.memory[pos + 1] as usize] = self.decode(pos + 2);
                Some(pos + 3)
            }
            2 => {
                let x = self.decode(pos + 1);
                self.stack.push(x);
                Some(pos + 2)
            }
            3 => {
                let x = self.stack.pop();
                self.memory[self.memory[pos + 1] as usize] = x.expect("Cannot pop from stack!");
                Some(pos + 2)
            }
            4 => {
                self.memory[self.memory[pos + 1] as usize] =
                    if self.decode(pos + 2) == self.decode(pos + 3) {
                        1
                    } else {
                        0
                    };
                Some(pos + 4)
            }
            5 => {
                self.memory[self.memory[pos + 1] as usize] =
                    if self.decode(pos + 2) > self.decode(pos + 3) {
                        1
                    } else {
                        0
                    };
                Some(pos + 4)
            }
            6 => Some(self.decode(pos + 1) as usize),
            7 => {
                if self.decode(pos + 1) != 0 {
                    Some(self.decode(pos + 2) as usize)
                } else {
                    Some(pos + 3)
                }
            }
            8 => {
                if self.decode(pos + 1) == 0 {
                    Some(self.decode(pos + 2) as usize)
                } else {
                    Some(pos + 3)
                }
            }
            9 => {
                let res = self.decode(pos + 2).wrapping_add(self.decode(pos + 3)) % (1 << 15);
                self.memory[self.memory[pos + 1] as usize] = res;
                Some(pos + 4)
            }
            10 => {
                let res = self.decode(pos + 2).wrapping_mul(self.decode(pos + 3)) % (1 << 15);
                self.memory[self.memory[pos + 1] as usize] = res;
                Some(pos + 4)
            }
            11 => {
                let res = self.decode(pos + 2) % self.decode(pos + 3);
                self.memory[self.memory[pos + 1] as usize] = res;
                Some(pos + 4)
            }
            12 => {
                let res = self.decode(pos + 2) & self.decode(pos + 3);
                self.memory[self.memory[pos + 1] as usize] = res;
                Some(pos + 4)
            }
            13 => {
                let res = self.decode(pos + 2) | self.decode(pos + 3);
                self.memory[self.memory[pos + 1] as usize] = res;
                Some(pos + 4)
            }
            14 => {
                let res = (!self.decode(pos + 2)) & 0x7fff_u16;
                self.memory[self.memory[pos + 1] as usize] = res;
                Some(pos + 3)
            }
            15 => {
                self.memory[self.memory[pos + 1] as usize] = self.memory[self.decode(pos + 2) as
                                                                             usize];
                Some(pos + 3)
            }
            16 => {
                self.memory[self.decode(pos + 1) as usize] = self.decode(pos + 2);
                Some(pos + 3)
            }
            17 => {
                self.stack.push((pos + 2) as u16);
                Some(self.decode(pos + 1) as usize)
            }
            18 => self.stack.pop().map_or(None, |x| Some(x as usize)),
            19 => {
                print!("{}", self.decode(pos + 1) as u8 as char);
                Some(pos + 2)
            }
            20 => {
                let mut input = [0];

                let stdin = stdin();
                assert!(stdin.lock().take(1).read(&mut input).is_ok());
                self.memory[self.memory[pos + 1] as usize] = input[0] as u16;
                Some(pos + 2)
            }
            _ => Some(pos + 1),
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
            &self.memory[1 << 15..8 + (1 << 15)],
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
        for (i, &x) in [1, 32768, 42, 4, 32769, 32768, 42, 5, 32770, 32768, 32769]
            .iter()
            .enumerate()
        {
            vm.memory[i] = x;
        }
        vm.run();
        assert_eq!(vm.memory[1 << 15], 42);
        assert_eq!(vm.memory[1 + (1 << 15)], 1);
        assert_eq!(vm.memory[2 + (1 << 15)], 1);
    }

    #[test]
    fn stack_op() {
        let mut vm = VirtualMachine::new();
        for (i, &x) in [1, 32768, 42, 2, 32768, 3, 32769].iter().enumerate() {
            vm.memory[i] = x;
        }
        vm.run();
        assert_eq!(vm.memory[1 + (1 << 15)], 42);
    }

    #[test]
    fn math_op() {
        let mut vm = VirtualMachine::new();
        for (i, &x) in [
            1,
            32768,
            42,
            9,
            32769,
            32768,
            32760,
            10,
            32770,
            32768,
            32000,
            11,
            32771,
            32770,
            5,
        ].iter()
            .enumerate()
        {
            vm.memory[i] = x;
        }
        vm.run();
        assert_eq!(vm.memory[32769], 34);
        assert_eq!(vm.memory[32770], 512);
        assert_eq!(vm.memory[32771], 2);
    }

    #[test]
    fn bitwise_op() {
        let mut vm = VirtualMachine::new();
        for (i, &x) in [
            12,
            32769,
            0xab,
            0x42,
            13,
            32770,
            0xab,
            0x42,
            14,
            32771,
            0x43ab,
        ].iter()
            .enumerate()
        {
            vm.memory[i] = x;
        }
        vm.run();
        assert_eq!(vm.memory[32769], 2);
        assert_eq!(vm.memory[32770], 0xeb);
        assert_eq!(vm.memory[32771], 0x3c54);
    }

    #[test]
    fn pointer_op() {
        let mut vm = VirtualMachine::new();
        for (i, &x) in [45, 1, 32769, 0, 15, 32770, 32769, 16, 32770, 27]
            .iter()
            .enumerate()
        {
            vm.memory[i] = x;
        }
        vm.run();
        assert_eq!(vm.memory[32770], 45);
        assert_eq!(vm.memory[45], 27);
    }
}
