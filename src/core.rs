extern crate sdl2;

use cpu::Cpu;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::Sdl;

pub struct Core {
    pub canvas: Canvas<Window>,
}

impl Core {

    // Initializes Core
    pub fn initialize(sdl_context: &Sdl, scale: u32) -> Core {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("Rusty CHIP-8", 64 * scale, 32 * scale)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        Core {
            canvas: canvas
        }
    }

    // Initializes user input
    pub fn initialize_input(&mut self) {
        println!("Input Initialized");
    }

    pub fn draw_canvas(&mut self, cpu: &mut Cpu, scale: u32) {
        for i in 0..64 * 32 {
            let current_pixel = cpu.display[i];
            let x = (i % 64) * scale as usize;
            let y = (i / 64) * scale as usize;
            
            if current_pixel == 1 {
                self.canvas.set_draw_color(Color::RGB(255, 255, 255));
            } else {
                self.canvas.set_draw_color(Color::RGB(0, 0, 0));
            }
            let _ = self.canvas.fill_rect(Rect::new(x as i32, y as i32, scale, scale));
        }
        self.canvas.present();
    }
}