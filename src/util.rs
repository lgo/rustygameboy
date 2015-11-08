use cpu::Cpu;
use opcode::opcode::Opcode;

pub static mut DEBUG_MODE: bool = false;

pub fn debug_cycle(cpu: &Cpu, opcode: &Opcode) {
    unsafe {
        if !DEBUG_MODE { return }
    }
    println!("cpu:");
    // for i in 0..16 {
    //     if cpu.v[i] == 0 { continue; }
    //     println!("      v{:.x}: {:.x}", i, cpu.v[i]);
    // }
    println!("    pc: {:.x}", cpu.pc);
    // println!("    I: {:.x}", cpu.i);
    println!("    sp: {:.x}", cpu.sp);
    println!("opcode: {:.x}", opcode.code);
    println!("opcode: {}", opcode.get_name());
}
