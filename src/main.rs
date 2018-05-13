extern crate rand;
mod core;
mod cpu;
mod instructions;
use cpu::Cpu;
use std::env;
use std::io;

fn main() {
    core::initialize_graphics();
    core::initialize_input();

    let mut cpu = Cpu::initialize();
    cpu.load_fontset();
    let args: Vec<String> = env::args().collect();
    cpu.load_game(&args[1]);
    loop {
        let mut test = String::new();
        println!("{:04X}", cpu.opcode);
        println!("Registers:");
        for i in 0..0xF + 1 {
            println!("V{:X}: {}", i, cpu.registers[i]);
        }
        println!("I: {}", cpu.index_register);
        for i in 0..64 * 32 {
            if i % 64 == 0 {
                println!("");
            }
            print!("{}", cpu.display[i]);
        }
        println!("");
        cpu.execute_cycle();
        io::stdin().read_line(&mut test).expect("");
    }
}
