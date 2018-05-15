extern crate rand;
extern crate sdl2;

mod core;
mod cpu;
mod instructions;

use cpu::Cpu;
use core::Core;
use std::env;
use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let args: Vec<String> = env::args().collect();
    let mut cpu = Cpu::initialize();
    let mut core = Core::initialize(&sdl_context, 10);

    cpu.load_fontset();
    cpu.load_game(&args[1]);

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(keycode), .. } => core.handle_key_down(&mut cpu, keycode),
                Event::KeyUp { keycode: Some(keycode), .. } => core.handle_key_up(&mut cpu, keycode),
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 1024));
        cpu.execute_cycle();
        if cpu.will_draw {
            core.draw_canvas(&mut cpu, 10);
        }
    }
}
