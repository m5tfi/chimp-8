#![warn(clippy::pedantic, clippy::all)]
use chimp_core::Vm;
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window,
    EventPump, Sdl,
};
use std::env;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

struct App {
    sdl_context: Sdl,
    canvas: Canvas<Window>,
    vm: Vm,
    is_running: bool,
}

// --- Constants ---
impl App {
    const SCALE: u32 = 15;
    #[allow(clippy::cast_possible_truncation)]
    const WINDOW_WIDTH: u32 = (Vm::SCREEN_WIDTH as u32) * Self::SCALE;
    #[allow(clippy::cast_possible_truncation)]
    const WINDOW_HEIGHT: u32 = (Vm::SCREEN_HEIGHT as u32) * Self::SCALE;
    const TICKS_PER_FRAME: usize = 10;
}

// --- Methods ---
impl App {
    pub fn new(file_name: &str, file_bytes: &[u8]) -> Result<Self> {
        let mut vm = Vm::default();
        vm.load_program(file_bytes);

        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let title = format!("Chimp-8 - {}", file_name);

        let window = video_subsystem
            .window(&title, Self::WINDOW_WIDTH, Self::WINDOW_HEIGHT)
            .position_centered()
            .build()?;

        let canvas = window.into_canvas().present_vsync().build()?;

        Ok(Self {
            sdl_context,
            canvas,
            vm,
            is_running: true,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        let mut event_pump = self.sdl_context.event_pump()?;
        while self.is_running {
            self.process_events(&mut event_pump);
            for _ in 0..Self::TICKS_PER_FRAME {
                self.vm.tick();
            }
            self.vm.tick_timers();
            self.draw_screen();
        }

        Ok(())
    }

    fn process_events(&mut self, event_pump: &mut EventPump) {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.is_running = false,
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    if let Some(k) = Self::keycode_to_hex(key) {
                        self.vm.keypress(k, true)
                    }
                }
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    if let Some(k) = Self::keycode_to_hex(key) {
                        self.vm.keypress(k, false)
                    }
                }
                _ => (),
            }
        }
    }

    #[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
    fn draw_screen(&mut self) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        let screen_buf = self.vm.get_display();
        self.canvas.set_draw_color(Color::WHITE);
        for (i, pixel) in screen_buf.iter().enumerate() {
            if *pixel {
                let x = (i % Vm::SCREEN_WIDTH) as u32;
                let y = (i / Vm::SCREEN_WIDTH) as u32;

                let rect = Rect::new(
                    (x * Self::SCALE) as i32,
                    (y * Self::SCALE) as i32,
                    Self::SCALE,
                    Self::SCALE,
                );
                self.canvas.fill_rect(rect).unwrap();
            }
        }
        self.canvas.present();
    }

    fn keycode_to_hex(key: Keycode) -> Option<usize> {
        match key {
            Keycode::Num1 => Some(0x1),
            Keycode::Num2 => Some(0x2),
            Keycode::Num3 => Some(0x3),
            Keycode::Num4 => Some(0xC),

            Keycode::Q => Some(0x4),
            Keycode::W => Some(0x5),
            Keycode::E => Some(0x6),
            Keycode::R => Some(0xD),

            Keycode::A => Some(0x7),
            Keycode::S => Some(0x8),
            Keycode::D => Some(0x9),
            Keycode::F => Some(0xE),

            Keycode::Z => Some(0xA),
            Keycode::X => Some(0x0),
            Keycode::C => Some(0xB),
            Keycode::V => Some(0xF),

            _ => None,
        }
    }
}

fn parse_args() -> Result<(String, Vec<u8>)> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        return Err(Error::from(
            "Invalid arguments!\n\
             Usage: chimp_desktop path/to/rom/file",
        ));
    }

    let file_name = args[1].clone();
    let file_bytes = match std::fs::read(&file_name) {
        Ok(bytes) => bytes,
        Err(e) => return Err(Box::new(e)),
    };

    Ok((file_name, file_bytes))
}

fn main() {
    let (file_name, file_bytes) = match parse_args() {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    let mut app = App::new(&file_name, &file_bytes).unwrap();
    app.run().unwrap();
}
