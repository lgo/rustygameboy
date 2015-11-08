#![allow(dead_code)]

use opcode::cycles;
use opcode::names;

pub struct Opcode {
    pub is_cb: bool,
    pub code: u8,
}

impl Opcode {
    pub fn new(code: u8, cb: bool) -> Opcode {
        return Opcode {
            code: code,
            is_cb: cb
        }
    }

    pub fn get_reg_1(&self) {
        // TODO
    }

    pub fn get_reg_2(&self) -> u8 {
        return (self.code & 0x0F) % 8;
    }

    pub fn get_cycles(&self, conditional: bool) -> u64 {
        if self.is_cb {
            cycles::OPCODE_CB_CYCLES[self.code as usize] as u64
        }
        else if conditional {
            cycles::OPCODE_CONDITIONAL_CYCLES[self.code as usize] as u64
        }
        else {
            cycles::OPCODE_CYCLES[self.code as usize] as u64
        }
    }

    pub fn get_name(&self) -> &'static str {
        if self.is_cb {
            names::OPCODE_CB_NAMES[self.code as usize]
        }
        else {
            names::OPCODE_NAMES[self.code as usize]
        }
    }
}
