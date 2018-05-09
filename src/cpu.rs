use std::fs::File;
use std::io::Read;

pub struct Cpu {
    program_counter: u16,
    opcode: u16,
    index_register: u16,
    stack: [u16; 16],
    stack_pointer: u16,
    sound_timer: u8,
    delay_timer: u8,
    registers: [u8; 16],
    key_state: [u8; 16],
    memory: [u8; 4096]
}

impl Cpu {

    // Default Constructor
    pub fn initialize() -> Cpu {
        Cpu {
            program_counter: 0x200,
            opcode: 0,
            index_register: 0,
            stack: [0; 16],
            stack_pointer: 0,
            sound_timer: 0,
            delay_timer: 0,
            registers: [0; 16],
            key_state: [0; 16],
            memory: [0; 4096]
        }
    }

    pub fn load_fontset(&mut self) {
        let fontset: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];

        for i in 0..80 {
            self.memory[i] = fontset[i];
        }
    }

    // Loads a game given the location of the file
    pub fn load_game(&mut self, location: &str) {
        let mut game = File::open(location).expect("Game was not found.");
        let mut buffer = [0; 3584];
        let buffer_size = game.read(&mut buffer[..]).expect("Error when reading file");
        
        for i in 0..buffer_size {
            self.memory[i + 512] = buffer[i];
        }
    }

    // Executes a single CPU cycle
    pub fn execute_cycle() {

    }
}