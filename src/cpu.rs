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