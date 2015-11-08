#![allow(dead_code)]

use opcode::opcode::Opcode;
use input::Input;
use display::Display;
use util as util;

fn not_implemented(msg: &str) {
    println!("{}", msg);
}

pub struct Cpu {
    pub input: Input,
    pub display: Display,

    pub stack: [u16; 16],

    pub memory: [u8; 4096],

    pub sp: usize,
    pub pc: usize,

    pub registers: [u16; 8],
}

fn truncate(n: u16) -> u16 {
    return n & 0x00FF;
}

impl Cpu {

    pub fn new() -> Cpu {
        let cpu: Cpu = Cpu {
            stack: [0; 16],
            memory: [0; 4096],
            pc: 0, // Program counter
            sp: 0, // stack pointer
            registers: [0; 8],
            display: Display::new(),
            input: Input::new()
        };
        return cpu;
    }

    pub fn get_register(&mut self, register: u8) -> u16 {
        match register {
            0x0 => self.registers[2], // b
            0x1 => self.registers[3], // c
            0x2 => self.registers[4], // d
            0x3 => self.registers[5], // e
            0x4 => self.registers[6], // h
            0x5 => self.registers[7], // l
            0x6 => self.registers[6] << 8 | self.registers[7], // hl
            0x7 => self.registers[0], // a
            _   => {
                not_implemented("Register out of range");
                0u16
            }
        }
    }

    pub fn set_register(&mut self, register: u8, value: u16) {
        match register {
            0x0 => { // b
                self.registers[2] = truncate(value)
            },
            0x1 => {
                self.registers[3] = truncate(value)
            }, // c
            0x2 => {
                self.registers[4] = truncate(value)
            }, // d
            0x3 => {
                self.registers[5] = truncate(value)
            }, // e
            0x4 => {
                self.registers[6] = truncate(value)
            }, // h
            0x5 => {
                self.registers[7] = truncate(value)
            }, // l
            0x6 => { // hl
                self.registers[6] = truncate(value) >> 8;
                self.registers[7] = truncate(value);
            },
            0x7 => self.registers[0] = truncate(value), // a
            _   => { not_implemented("Register out of range") }
        };
    }

    pub fn fetch_opcode (&mut self) -> Opcode {
        let first_byte = self.memory[self.pc];
        if first_byte == 0xCB {
            let second_byte = self.memory[self.pc + 1];
            return Opcode::new(second_byte, true);
        }
        else {
            return Opcode::new(first_byte, false);
        }
    }


    pub fn execute_opcode (&mut self, opcode: &Opcode) -> u64 {
        util::debug_cycle(self, &opcode);
        let cycles_used = 10;
        return cycles_used;
    }
}
