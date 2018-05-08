mod core;
mod cpu;
use cpu::Cpu;

fn main() {
    core::initialize_graphics();
    core::initialize_input();

    let cpu = Cpu::initialize();
    cpu.load_game("Tetris");
}
