use std::collections::HashMap;

use sdl2::{pixels::Color, event::Event, keyboard::Scancode, video::Window, render::Canvas, EventPump, rect::Rect, audio::{AudioCallback, AudioSpecDesired, AudioDevice}};

use super::Drawable;

extern crate sdl2;

pub struct SDLGraphics {
    width_cells: u32,
    height_cells: u32,
    pixel_size: u32,
    canvas: Canvas<Window>,
    event_pump: EventPump,
    keymap: HashMap<u8, char>,
    close_requested: bool,
    audio_device: AudioDevice<SquareWave>,
}

impl SDLGraphics {
    pub fn new(width_cells: u32, height_cells: u32, pixel_size: u32, keymap: HashMap<u8, char>) -> SDLGraphics {
        let ctx = sdl2::init().unwrap();
        let video = ctx.video().unwrap();
        let window = video.window("Chip8", width_cells * pixel_size, height_cells * pixel_size)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas()
            .build()
            .unwrap();
        let event_pump = ctx.event_pump().unwrap();
        let audio = ctx.audio().unwrap();
        let audio_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),
            samples: None
        };

        let audio_device = audio.open_playback(None, &audio_spec, |spec| {
            SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25
            }
        }).unwrap();

        SDLGraphics {
            width_cells,
            height_cells,
            pixel_size,
            canvas,
            event_pump,
            keymap,
            close_requested: false,
            audio_device,
        }
    }
}

impl Drawable for SDLGraphics {
    fn init(&mut self) {
        // Clear the screen
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.present();
    }

    fn finalize(&mut self) {
        // Do nothing
    }

    fn width(&self) -> usize {
        self.width_cells as usize
    }

    fn height(&self) -> usize {
        self.height_cells as usize
    }

    fn is_key_pressed(&self, key: u8) -> bool {
        let &keychar = self.keymap.get(&key).unwrap();
        self.event_pump.keyboard_state().is_scancode_pressed(keychar_to_scancode(keychar))
    }
    
    fn should_close(&self) -> bool {
        self.close_requested
    }

    fn wait_for_key(&mut self) -> u8 {
        loop {
            let event = self.event_pump.wait_event();

            match event {
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    for (key, keychar) in self.keymap.iter() {
                        if keycode.name().to_ascii_lowercase() == String::from(*keychar).to_ascii_lowercase() {
                            return *key;
                        }
                    }
                },
                Event::Quit { .. } => {
                    self.close_requested = true;
                    return 42
                }
                _ => (),
            }
        }
    }

    fn draw_screen(&mut self, vram: &Vec<Vec<u8>>) {
        for (y, row) in vram.iter().enumerate() {
            for (x, &pixel) in row.iter().enumerate() {
                if pixel != 0 {
                    self.canvas.set_draw_color(Color::RGB(255, 255, 255));
                } else {
                    self.canvas.set_draw_color(Color::RGB(0, 0, 0));
                }

                self.canvas.fill_rect(Rect::new(
                    (x * self.pixel_size as usize) as i32,
                    (y * self.pixel_size as usize) as i32,
                    self.pixel_size,
                    self.pixel_size
                )).expect("Failed to draw rectangle, possible driver failure");
            }
        }
        self.canvas.present();
    }

    fn update(&mut self, timeout_millis: u32) {
        // Use wait_event as to not busy-poll the event queue
        if let Some(event) = self.event_pump.wait_event_timeout(timeout_millis) {
            if let Event::Quit { .. } = event {
                self.close_requested = true;
            }

            // When we do receive an event, make sure to exhaust the whole event queue
            while let Some(event) = self.event_pump.poll_event() {
                if let Event::Quit { .. } = event {
                    self.close_requested = true;
                }
            }
        }
    }

    fn sound_resume(&self) {
        self.audio_device.resume();
    }

    fn sound_pause(&self) {
        self.audio_device.pause();
    }
}

fn keychar_to_scancode(keychar: char) -> Scancode {
    match keychar.to_ascii_lowercase() {
        '1' => Scancode::Num1,
        '2' => Scancode::Num2,
        '3' => Scancode::Num3,
        '4' => Scancode::Num4,
        'q' => Scancode::Q,
        'w' => Scancode::W,
        'e' => Scancode::E,
        'r' => Scancode::R,
        'a' => Scancode::A,
        's' => Scancode::S,
        'd' => Scancode::D,
        'f' => Scancode::F,
        'z' => Scancode::Z,
        'x' => Scancode::X,
        'c' => Scancode::C,
        'v' => Scancode::V,
        _ => panic!("Unsupported scancode")
    }
}

// Square wave generation taken from https://rust-sdl2.github.io/rust-sdl2/sdl2/audio/index.html
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