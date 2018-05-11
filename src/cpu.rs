use std::fs::File;
use std::io::Read;
use instructions::*;

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
        let most_significant_byte = self.opcode & 0xF000;

        match most_significant_byte {
            0x0000 => match self.opcode & 0x0FFF {
                0x00E0 => cls(self),
                0x00EE => ret(self),
                _ => sys_addr(self)
            },
            0x1000 => jp_addr(self),
            0x2000 => call_addr(self),
            0x3000 => se_vx_byte(self),
            0x4000 => sne_vx_byte(self),
            0x5000 => se_vx_vy(self),
            0x6000 => ld_vx_byte(self),
            0x7000 => add_vx_byte(self),
            0x8000 => match self.opcode & 0x000F {
                0x0000 => ld_vx_vy(self),
                0x0001 => or_vx_vy(self),
                0x0002 => and_vx_vy(self),
                0x0003 => xor_vx_vy(self),
                0x0004 => add_vx_vy(self),
                0x0005 => sub_vx_vy(self),
                0x0006 => shr_vx_vy(self),
                0x0007 => subn_vx_vy(self),
                0x000E => shl_vx_vy(self),
                _ => panic!("opcode {} was not recognized", self.opcode)
            },
            0x9000 => sne_vx_vy(self),
            0xA000 => ld_i_addr(self),
            0xB000 => jp_v0_addr(self),
            0xC000 => rnd_vx_byte(self),
            0xD000 => drw_vx_vy_n(self),
            0xE000 => match self.opcode & 0x00FF {
                0x009E => skp_vx(self),
                0x00A1 => sknp_vx(self),
                _ => panic!("opcode {} was not recognized", self.opcode)
            },
            0xF000 => match self.opcode & 0x00FF {
                0x0007 => ld_vx_dt(self),
                0x000A => ld_vx_k(self),
                0x0015 => ld_dt_vx(self),
                0x0018 => ld_st_vx(self),
                0x001E => add_i_vx(self),
                0x0029 => ld_f_vx(self),
                0x0033 => ld_b_vx(self),
                0x0055 => ld_i_vx(self),
                0x0065 => ld_vx_i(self),
                _ => panic!("opcode {} was not recognized", self.opcode)
            },
            _ => panic!("opcode {} was not recognized", self.opcode)
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