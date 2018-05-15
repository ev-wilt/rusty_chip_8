extern crate rand;

use cpu::Cpu;
use rand::Rng;

pub fn cls(cpu: &mut Cpu) {
    for i in 0..64*32 {
        cpu.display[i] = 0;
    }
    cpu.program_counter += 2;
}

pub fn ret(cpu: &mut Cpu) {
    cpu.stack_pointer -= 1;
    cpu.program_counter = cpu.stack[cpu.stack_pointer] as usize;
}

pub fn jp_addr(cpu: &mut Cpu) {
    cpu.program_counter = (cpu.opcode & 0x0FFF) as usize;
}

pub fn call_addr(cpu: &mut Cpu) {
    cpu.stack[cpu.stack_pointer] = (cpu.program_counter + 2) as u16;
    cpu.stack_pointer += 1;
    cpu.program_counter = (cpu.opcode & 0x0FFF) as usize;
}

pub fn se_vx_byte(cpu: &mut Cpu) {
    let vx = cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize];

    if vx == (cpu.opcode & 0x00FF) as u8 {
        cpu.program_counter += 4;
    } else {
        cpu.program_counter += 2;
    }
}

pub fn sne_vx_byte(cpu: &mut Cpu) {
    let vx = cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize];

    if vx != (cpu.opcode & 0x00FF) as u8 {
        cpu.program_counter += 4;
    } else {
        cpu.program_counter += 2;
    }
}

pub fn se_vx_vy(cpu: &mut Cpu) {
    let vx = cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize];
    let vy = cpu.registers[((cpu.opcode & 0x00F0) >> 4) as usize];

    if vx == vy {
        cpu.program_counter += 4;
    } else {
        cpu.program_counter += 2;
    }
}

pub fn ld_vx_byte(cpu: &mut Cpu) {
    cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize] = (cpu.opcode & 0x00FF) as u8;
    cpu.program_counter += 2;
}

pub fn add_vx_byte(cpu: &mut Cpu) {
    let vx = cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize] as u16;
    let sum = vx + (cpu.opcode & 0x00FF) as u16;

    cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize] = sum as u8;
    cpu.program_counter += 2;
}

pub fn ld_vx_vy(cpu: &mut Cpu) {
    let vy = cpu.registers[((cpu.opcode & 0x00F0) >> 4) as usize];

    cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize] = vy;
    cpu.program_counter += 2;
}

pub fn or_vx_vy(cpu: &mut Cpu) {
    let vx = cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize];
    let vy = cpu.registers[((cpu.opcode & 0x00F0) >> 4) as usize];

    cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize] = vx | vy;
    cpu.program_counter += 2;
}

pub fn and_vx_vy(cpu: &mut Cpu) {
    let vx = cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize];
    let vy = cpu.registers[((cpu.opcode & 0x00F0) >> 4) as usize];

    cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize] = vx & vy;
    cpu.program_counter += 2;
}

pub fn xor_vx_vy(cpu: &mut Cpu) {
    let vx = cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize];
    let vy = cpu.registers[((cpu.opcode & 0x00F0) >> 4) as usize];

    cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize] = vx ^ vy;
    cpu.program_counter += 2;
}

pub fn add_vx_vy(cpu: &mut Cpu) {
    let vx = cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize] as u16;
    let vy = cpu.registers[((cpu.opcode & 0x00F0) >> 4) as usize] as u16;
    let sum = vx + vy;

    cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize] = sum as u8;
    if sum > 0xFF {
        cpu.registers[0xF] = 1;
    } else {
        cpu.registers[0xF] = 0;
    }
    cpu.program_counter += 2;
}

pub fn sub_vx_vy(cpu: &mut Cpu) {
    let vx = cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize];
    let vy = cpu.registers[((cpu.opcode & 0x00F0) >> 4) as usize];

    if vx > vy {
        cpu.registers[0xF] = 1;
    } else {
        cpu.registers[0xF] = 0;
    }
    cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize] = vx.wrapping_sub(vy);    
    cpu.program_counter += 2;
}

pub fn shr_vx_vy(cpu: &mut Cpu) {
    let vx = cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize];

    cpu.registers[0xF] = vx & 0x01;
    cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize] >>= 1;
    cpu.program_counter += 2;
}

pub fn subn_vx_vy(cpu: &mut Cpu) {
    let vx = cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize];
    let vy = cpu.registers[((cpu.opcode & 0x00F0) >> 4) as usize];

    if vy > vx {
        cpu.registers[0xF] = 1;
    } else {
        cpu.registers[0xF] = 0;
    }
    cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize] = vy.wrapping_sub(vx);    
    cpu.program_counter += 2;
}

pub fn shl_vx_vy(cpu: &mut Cpu) {
    let vx = cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize];

    cpu.registers[0xF] = (vx & 0x80) >> 7;
    cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize] <<= 1;
    cpu.program_counter += 2;
}

pub fn sne_vx_vy(cpu: &mut Cpu) {
    let vx = cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize];
    let vy = cpu.registers[((cpu.opcode & 0x00F0) >> 4) as usize];

    if vx != vy {
        cpu.program_counter += 4;
    } else {
        cpu.program_counter += 2;
    }
}

pub fn ld_i_addr(cpu: &mut Cpu) {
    cpu.index_register = cpu.opcode & 0x0FFF;
    cpu.program_counter += 2;
}

pub fn jp_v0_addr(cpu: &mut Cpu) {
    cpu.program_counter += 0x0FFF + cpu.registers[0x0] as usize;
    cpu.program_counter += 2;
}

pub fn rnd_vx_byte(cpu: &mut Cpu) {
    let mask = cpu.opcode & 0x00FF;
    let random_num: u8 = rand::thread_rng().gen();

    cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize] = random_num & mask as u8;
}

pub fn drw_vx_vy_n(cpu: &mut Cpu) {
    let vx = cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize];
    let vy = cpu.registers[((cpu.opcode & 0x00F0) >> 4) as usize];
    let rows = cpu.opcode & 0x000F;
    
    for y in 0..rows {
        let pixel = cpu.memory[(cpu.index_register + y) as usize];

        for x in 0..8 {
            if (pixel & (0x80 >> x)) != 0 {
                if cpu.display[((vx + x) as u16 + ((vy as u16 + y) * 64)) as usize] == 1 {
                    cpu.registers[0xF] = 1;
                } else {
                    cpu.registers[0xF] = 0;
                }

                cpu.display[((vx + x) as u16 + ((vy as u16 + y) * 64)) as usize] ^= 1;
            }
        }
    }

    cpu.will_draw = true;
    cpu.program_counter += 2;
}

pub fn skp_vx(cpu: &mut Cpu) {
    let key = cpu.registers[((cpu.opcode & 0x0F00 ) >> 8) as usize];
    
    if cpu.key_state[key as usize] != false {
        cpu.program_counter += 4;
    } else {
        cpu.program_counter += 2;
    }
}

pub fn sknp_vx(cpu: &mut Cpu) {
    let key = cpu.registers[((cpu.opcode & 0x0F00 ) >> 8) as usize];
    
    if cpu.key_state[key as usize] != true {
        cpu.program_counter += 4;
    } else {
        cpu.program_counter += 2;
    }
}

pub fn ld_vx_dt(cpu: &mut Cpu) {
    cpu.registers[((cpu.opcode & 0x0F00 ) >> 8) as usize] = cpu.delay_timer;
    cpu.program_counter += 2;
}

pub fn ld_vx_k(cpu: &mut Cpu) {
    for i in 0..16 {
        if cpu.key_state[i] == true {
            cpu.registers[((cpu.opcode & 0x0F00 ) >> 8) as usize] = i as u8;
            cpu.program_counter += 2;
            break;
        }
    }
}

pub fn ld_dt_vx(cpu: &mut Cpu) {
    cpu.delay_timer = cpu.registers[((cpu.opcode & 0x0F00 ) >> 8) as usize];
    cpu.program_counter += 2;
}

pub fn ld_st_vx(cpu: &mut Cpu) {
    cpu.sound_timer = cpu.registers[((cpu.opcode & 0x0F00 ) >> 8) as usize];
    cpu.program_counter += 2;
}

pub fn add_i_vx(cpu: &mut Cpu) {
    let vx = cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize];

    cpu.index_register += vx as u16;
    cpu.program_counter += 2;
}

pub fn ld_f_vx(cpu: &mut Cpu) {
    let vx = cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize];

    cpu.index_register = (vx * 0x5) as u16;
    cpu.program_counter += 2;
}

pub fn ld_b_vx(cpu: &mut Cpu) {
    let vx = cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize];

    cpu.memory[cpu.index_register as usize] = vx / 100;
    cpu.memory[(cpu.index_register + 1) as usize] = (vx / 10) % 10;
    cpu.memory[(cpu.index_register + 2) as usize] = (vx % 100) % 10;
    cpu.program_counter += 2;
}

pub fn ld_i_vx(cpu: &mut Cpu) {
    let x = (cpu.opcode & 0x0F00) >> 8;

    for i in 0..x + 1 {
        cpu.memory[(cpu.index_register + i) as usize] = cpu.registers[i as usize];
    }
    cpu.program_counter += 2;
}

pub fn ld_vx_i(cpu: &mut Cpu) {
    let x = (cpu.opcode & 0x0F00) >> 8;

    for i in 0..x + 1 {
        cpu.registers[i as usize] = cpu.memory[(cpu.index_register + i) as usize];
    }
    cpu.program_counter += 2;
}