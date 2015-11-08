use std::io::prelude::*;
use std::fs::File;

use cpu::Cpu;

pub fn load_game(cpu: &mut Cpu, game: String) {
    // let mut path = os::getcwd().unwrap();
    // path.push(game.trim());
    let mut reader = File::open(format!("games/{}", game)).unwrap();
    load_to_memory(cpu, &mut reader);
}

fn load_to_memory(cpu: &mut Cpu, reader: &mut File) {
    for byte in reader.bytes() {
        match byte {
            Ok(value) => {
                cpu.memory[cpu.pc] = value;
                cpu.pc += 1;
            }
            Err(_)   => {
                break;
            }
        }
    }
    cpu.pc = 0x200;
}
