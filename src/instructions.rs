#![allow(non_snake_case)]

use instruction;
use cpu::Cpu as Cpu;

pub fn opcode_execute(cpu: &mut Cpu, opcode: &Instruction) {
    cpu.pc += 2;
    match opcode.code & 0xf000 {
        0x0000 => op_0xxx(cpu, opcode),
        0x1000 => op_1xxx(cpu, opcode),
        0x2000 => op_2xxx(cpu, opcode),
        0x3000 => op_3xxx(cpu, opcode),
        0x4000 => op_4xxx(cpu, opcode),
        0x5000 => op_5xxx(cpu, opcode),
        0x6000 => op_6xxx(cpu, opcode),
        0x7000 => op_7xxx(cpu, opcode),
        0x8000 => op_8xxx(cpu, opcode),
        0x9000 => op_9xxx(cpu, opcode),
        0xA000 => op_Axxx(cpu, opcode),
        0xB000 => op_Bxxx(cpu, opcode),
        0xC000 => op_Cxxx(cpu, opcode),
        0xD000 => op_Dxxx(cpu, opcode),
        0xE000 => op_Exxx(cpu, opcode),
        0xF000 => op_Fxxx(cpu, opcode),
        _      => not_implemented(opcode.code as usize, cpu.pc)
    }
}

fn not_implemented(op: usize, pc: usize) {
    println!("Not implemented:: op: {:x}, pc: {:x}", op, pc);
}

fn op_0xxx(cpu: &mut Cpu, opcode: &Opcode) {
    match opcode.code {
        0x00E0 => {
            // CLS
            // clear screen
            cpu.display.clear();
        },
        0x00EE => {
            // RET
            // returns from subroutine
            cpu.sp -= 1;
        },
        _      => { not_implemented(opcode.code as usize, cpu.pc) }
    }
}
fn op_1xxx(cpu: &mut Cpu, opcode: &Opcode) {
    // JP addr
    // jumps to addr
    cpu.pc = opcode.address;
}

fn op_2xxx(cpu: &mut Cpu, opcode: &Opcode) {
    // CALL addr
    // calls subroutine at addr
    cpu.stack[cpu.sp] = opcode.address as u16;
    cpu.sp += 1;
}

fn op_3xxx(cpu: &mut Cpu, opcode: &Opcode) {
    // SE Vx, byte
    // skips next instruction if VX eq NN
    if cpu.v[opcode.x] == opcode.const8bit {
        cpu.pc += 2;
    }
}

fn op_4xxx(cpu: &mut Cpu, opcode: &Opcode) {
    // SNE Vx, byte
    // skips next instruction if VX neq MM
    if cpu.v[opcode.x] != opcode.const8bit {
        cpu.pc += 2;
    }
}

fn op_5xxx(cpu: &mut Cpu, opcode: &Opcode) {
    // SE Vx, Vy
    // skips next instruction if VX eq VY
    if cpu.v[opcode.x] != opcode.const8bit {
        cpu.pc += 2;
    }
}

fn op_6xxx(cpu: &mut Cpu, opcode: &Opcode) {
    // LD Vx, byte
    // sets VX to NN
    cpu.v[opcode.x] = opcode.const8bit;
}

fn op_7xxx(cpu: &mut Cpu, opcode: &Opcode) {
    // ADD Vx, byte
    // incr VX by NN
    cpu.v[opcode.x] += opcode.const8bit;
}

fn op_8xxx(cpu: &mut Cpu, opcode: &Opcode) {
    match opcode.code & 0x000F {
        0x0000 => {
            // LD Vx, Vy
            // set VX to VY
            cpu.v[opcode.x] += cpu.v[opcode.y];
        },
        0x0001 => {
            // OR Vx, Vy
            // sets VX to VX or VY
            cpu.v[opcode.x] |= cpu.v[opcode.y];
        },
        0x0002 => {
            // AND Vx, Vy
            // sets VX to VX and VY
            cpu.v[opcode.x] &= cpu.v[opcode.y];
        },
        0x0003 => {
            // XOR Vx, Vy
            // sets VX to VX and VY
            cpu.v[opcode.x] ^= cpu.v[opcode.y];
        },
        0x0004 => {
            // ADD Vx, Vy
            // incr VX by VY, if VF = 1 if carry else 0
            if cpu.v[opcode.y] < (0xFF - cpu.v[opcode.x]) {
                cpu.v[0xF] = 1;
            }
            else {
                cpu.v[0xF] = 0;
            };

            cpu.v[opcode.x] = (cpu.v[opcode.x] as u16 + cpu.v[opcode.y] as u16) as u8;
        },
        0x0005 => {
            // SUB Vx, Vy
            // sub VX by VY, if VF = 0 if borrow else 1
            if cpu.v[opcode.y] > cpu.v[opcode.x] {
                cpu.v[0xF] = 1;
            }
            else {
                cpu.v[0xF] = 0;
            }

            cpu.v[opcode.x] -= cpu.v[opcode.y];
        },
        0x0006 => {
            // SHR Vx {, Vy}
            // right ship VX by 1. Set VF to the least significant bit before shifting
            cpu.v[0xF] = cpu.v[opcode.x] & 0x0001;
            cpu.v[opcode.x] >> 1;
        },
        0x0007 => {
            // SUBN Vx, Vy
            // sub VX by VY, if VF = 0 if borrow else 1
            if cpu.v[opcode.x] > cpu.v[opcode.y] {
                cpu.v[0xF] = 1;
            }
            else {
                cpu.v[0xF] = 0;
            }

            cpu.v[opcode.x] = cpu.v[opcode.y] - cpu.v[opcode.x];
        },
        0x000E => {
            // SHL Vx {, Vy}
            // Set Vx = Vx SHL 1.
            // If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0.
            // Then Vx is multiplied by 2.
            // TODO
        }
        _      => { not_implemented(opcode.code as usize, cpu.pc) }
    }
}

fn op_9xxx(cpu: &mut Cpu, opcode: &Opcode) {
    // SNE Vx, Vy
    // skips the next instruction if VX neq Vy
    if cpu.v[opcode.x] != cpu.v[opcode.y] {
        cpu.pc += 2;
    }
}

fn op_Axxx(cpu: &mut Cpu, opcode: &Opcode) {
    // LD I, addr
    // sets I to addr
    cpu.i = opcode.address;
}

fn op_Bxxx(cpu: &mut Cpu, opcode: &Opcode) {
    // JP V0, addr
    // jumps to address + V0
    cpu.pc = opcode.address+ cpu.v[0x0] as usize;
}

fn op_Cxxx(cpu: &mut Cpu, opcode: &Opcode) {
    // RND Vx, byte
    // sets VX to a random number, masked by NN
    let rand = 0;
    cpu.v[opcode.x] = rand & opcode.const8bit;
}

fn op_Dxxx(cpu: &mut Cpu, opcode: &Opcode) {
    // DRW Vx, Vy, nibble
    // Sprites stored in memory at location in index register (I), maximum 8bits wide.
    // Wraps around the screen. If when drawn, clears a pixel,
    // register VF is set to 1 otherwise it is zero.
    // All drawing is XOR drawing (i.e. it toggles the screen pixels)

    let from: usize = cpu.i;
    let to: usize = from + opcode.const4bit as usize;
    let x: u8 = cpu.v[opcode.x];
    let y: u8 = cpu.v[opcode.y];
    cpu.v[15] = cpu.display.draw(x as usize, y as usize, &cpu.memory[from .. to]);
    cpu.pc += 2;


    cpu.v[0xF] = 0;

    let height = opcode.code & 0x000F;
    let regX = cpu.v[opcode.x] as usize;
    let regY = cpu.v[opcode.y] as usize;
    let mut spr;

    for y in 0..height {
        spr = cpu.memory[cpu.i + y as usize];
        for x in 0..8 {
            if (spr & 0x80) > 0 {
                cpu.display.gfx[regX + x][regY + y as usize] = cpu.display.gfx[regX + x][regY + y as usize] ^ 1;
                if cpu.display.gfx[regX + x as usize][regY + y as usize] == 1{
                    cpu.v[0xF] = 1;
                }
            }
            spr <<= 1;
        }
    }
    cpu.display.draw_flag = true;
}

fn op_Exxx(cpu: &mut Cpu, opcode: &Opcode) {
    match opcode.code & 0x00FF {
        0x009E => {
            // SKP Vx
            // Skips the next instruction if the key stored in VX is pressed.
            if cpu.keypad.pressed(cpu.v[opcode.x] as usize) {
                cpu.pc += 2;
            }
        },
        0x00A1 => {
            // SKNP Vx
            // Skips the next instruction if the key stored in VX isn't pressed.
            if !cpu.keypad.pressed(cpu.v[opcode.x] as usize) {
                cpu.pc += 2;
            }
        },
        _      => { not_implemented(opcode.code as usize, cpu.pc) }
    }
}

fn op_Fxxx(cpu: &mut Cpu, opcode: &Opcode) {
    match opcode.code & 0x00FF {
        0x0007 => {
            // LD Vx, DT
            // Sets VX to the value of the delay timer.
            cpu.v[opcode.x] = cpu.delay_timer;
        },
        0x000A => {
            // LD Vx, K
            // A key press is awaited, and then stored in VX.
            for i in 0u8..16 {
                if cpu.keypad.pressed(i as usize) {
                    cpu.v[opcode.x] = i;
                    break;
                }
            }
            cpu.pc -= 2;
        },
        0x0015 => {
            // LD DT, Vx
            // Sets the delay timer to VX.
            cpu.delay_timer = cpu.v[opcode.x];
        },
        0x0018 => {
            // LD ST, Vx
            // Sets the sound timer to VX.
            cpu.sound_timer = cpu.v[opcode.x];
        },
        0x001E => {
            // ADD I, Vx
            // adds VX to I

            // VF is set to 1 when range overflow (I+VX>0xFFF), and 0 when there isn't.
            // This is undocumented feature of the CHIP-8 and used by Spacefight 2091! game.
            if cpu.i as u16 + (cpu.v[opcode.x] as u16) > 0xFFF {
                cpu.v[0xF] = 1;
            }
            else {
                cpu.v[0xF] = 0;
            }

            cpu.i += cpu.v[opcode.x] as usize;
        },
        0x0029 => {
            // LD F, Vx
            // Sets I to the location of the sprite for the character in VX.
            // Characters 0-F (in hexadecimal) are represented by a 4x5 font.

        },
        0x0033 => {
            // LD B, Vx
            // Stores the Binary-coded decimal representation of VX,
            // with the most significant of three digits at the address in I,
            // the middle digit at I plus 1, and the least significant digit at I plus 2.
            // (In other words, take the decimal representation of VX, place the hundreds
            // digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.)
        },
        0x0055 => {
            // LD [I], Vx
            // Stores V0 to VX in memory starting at address I.
            for i in 0..cpu.v[opcode.x] {
                cpu.memory[cpu.i + i as usize] = cpu.v[i as usize];
            }
        },
        0x0065 => {
            // LD Vx, [I]
            // Fills V0 to VX with values from memory starting at address I.
            for i in 0..cpu.v[opcode.x] {
                cpu.v[i as usize] = cpu.memory[cpu.i + i as usize];
            }
        },
        _      => { not_implemented(opcode.code as usize, cpu.pc) }
    }
}
