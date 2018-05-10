use std::fs::File;
use std::io::Read;

pub struct Cpu {
    program_counter: usize,
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

    // Loads the Chip-8 fontset into memory
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
        let mut game = File::open(location).expect("Game was not found");
        let mut buffer = [0; 3584];
        let buffer_size = game.read(&mut buffer[..]).expect("Error when reading file");
        
        for i in 0..buffer_size {
            self.memory[i + 512] = buffer[i];
        }
    }

    // Interprets an opcode and runs the necessary code for it
    pub fn interpret_opcode(&mut self) {
        let most_significant_bit = self.opcode & 0xF000;
        
        if most_significant_bit == 0x0000 {

        } else if most_significant_bit == 0x1000 {

        } else if most_significant_bit == 0x2000 {

        } else if most_significant_bit == 0x3000 {

        } else if most_significant_bit == 0x4000 {

        } else if most_significant_bit == 0x5000 {

        } else if most_significant_bit == 0x6000 {

        } else if most_significant_bit == 0x7000 {

        } else if most_significant_bit == 0x8000 {

        } else if most_significant_bit == 0x9000 {

        } else if most_significant_bit == 0xA000 {

        } else if most_significant_bit == 0xB000 {

        } else if most_significant_bit == 0xC000 {

        } else if most_significant_bit == 0xD000 {

        } else if most_significant_bit == 0xF000 {

        } else {
            panic!("Opcode {} was undefined", self.opcode);
        }
    }

    // Executes a single CPU cycle
    pub fn execute_cycle(&mut self) {

        // Build opcode with next two bytes
        self.opcode = (self.memory[self.program_counter] as u16) << 8 | self.memory[self.program_counter + 1] as u16;

        // Interpret opcode
        self.interpret_opcode();

        // Update timers
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                // Output beep
            }
            self.sound_timer -= 1;
        }
    }
}