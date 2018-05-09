mod core;
mod cpu;
use cpu::Cpu;
use std::env;

fn main() {
    core::initialize_graphics();
    core::initialize_input();

    let mut cpu = Cpu::initialize();
    let args: Vec<String> = env::args().collect();
    cpu.load_game(&args[1]);
}
