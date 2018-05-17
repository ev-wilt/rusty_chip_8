use cpu::Cpu;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::keyboard::Keycode;
use sdl2::audio::{AudioCallback, AudioSpecDesired, AudioDevice};
use sdl2::Sdl;

pub struct Core {
    pub canvas: Canvas<Window>,
    device: AudioDevice<SquareWave>
}

impl Core {

    /// Initializes Core
    pub fn initialize(sdl_context: &Sdl, scale: u32) -> Core {

        // Set up audio
        let audio_subsystem = sdl_context.audio().unwrap();
        let desired_spec = AudioSpecDesired {
            freq: Some(44000),
            channels: Some(1),  // mono
            samples: None       // default sample size
        };

        let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
            // Initialize the audio callback
            SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25
            }
        }).unwrap();

        // Set up video
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
            canvas: canvas,
            device: device
        }
    }

    /// Handles key down event
    pub fn handle_key_down(&mut self, cpu: &mut Cpu, keycode: Keycode) {
        match keycode {
            Keycode::Num1 => { cpu.key_state[0x1] = true; },
            Keycode::Num2 => { cpu.key_state[0x2] = true; },
            Keycode::Num3 => { cpu.key_state[0x3] = true; },
            Keycode::Num4 => { cpu.key_state[0xC] = true; },
            Keycode::Q => { cpu.key_state[0x4] = true; },
            Keycode::W => { cpu.key_state[0x5] = true; },
            Keycode::E => { cpu.key_state[0x6] = true; },
            Keycode::R => { cpu.key_state[0xD] = true; },
            Keycode::A => { cpu.key_state[0x7] = true; },
            Keycode::S => { cpu.key_state[0x8] = true; },
            Keycode::D => { cpu.key_state[0x9] = true; },
            Keycode::F => { cpu.key_state[0xE] = true; },
            Keycode::Z => { cpu.key_state[0xA] = true; },
            Keycode::X => { cpu.key_state[0x0] = true; },
            Keycode::C => { cpu.key_state[0xB] = true; },
            Keycode::V => { cpu.key_state[0xF] = true; },
            _ => {}
        }
    }

    /// Handles key up event
    pub fn handle_key_up(&mut self, cpu: &mut Cpu, keycode: Keycode) {
        match keycode {
            Keycode::Num1 => { cpu.key_state[0x1] = false; },
            Keycode::Num2 => { cpu.key_state[0x2] = false; },
            Keycode::Num3 => { cpu.key_state[0x3] = false; },
            Keycode::Num4 => { cpu.key_state[0xC] = false; },
            Keycode::Q => { cpu.key_state[0x4] = false; },
            Keycode::W => { cpu.key_state[0x5] = false; },
            Keycode::E => { cpu.key_state[0x6] = false; },
            Keycode::R => { cpu.key_state[0xD] = false; },
            Keycode::A => { cpu.key_state[0x7] = false; },
            Keycode::S => { cpu.key_state[0x8] = false; },
            Keycode::D => { cpu.key_state[0x9] = false; },
            Keycode::F => { cpu.key_state[0xE] = false; },
            Keycode::Z => { cpu.key_state[0xA] = false; },
            Keycode::X => { cpu.key_state[0x0] = false; },
            Keycode::C => { cpu.key_state[0xB] = false; },
            Keycode::V => { cpu.key_state[0xF] = false; },
            _ => {}
        }
    }

    /// Draws the CPU's display to the canvas
    pub fn draw_canvas(&mut self, cpu: &mut Cpu, scale: u32) {
        for i in 0..64 * 32 {
            let current_pixel = cpu.display[i];
            let x = (i % 64) * scale as usize;
            let y = (i / 64) * scale as usize;
            
            self.canvas.set_draw_color(Color::RGB(0, 0, 0));
            if current_pixel == 1 {
                self.canvas.set_draw_color(Color::RGB(255, 255, 255));
            }
            let _ = self.canvas.fill_rect(Rect::new(x as i32, y as i32, scale, scale));
        }
        self.canvas.present();
    }

    /// Plays a beep sound
    pub fn play_sound(&mut self) {
        self.device.resume();
    }

    /// Stops the beep sound
    pub fn stop_sound(&mut self) {
        self.device.pause();
    }
}

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}