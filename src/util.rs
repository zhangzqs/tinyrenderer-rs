use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::{Dimensions, DrawTarget, Size},
};
use embedded_graphics_simulator::{sdl2::Keycode, OutputSettingsBuilder, SimulatorDisplay, Window};

use crate::draw_target::{Color, FrameBuffer};

pub struct DisplayWindow {
    pub fb: FrameBuffer<Color>,
    display: SimulatorDisplay<Rgb888>,
    window: Window,
}

pub enum Event {
    Nothing,
    Exit,
    Go,
    Back,
    TurnLeft,
    TurnRight,
    Up,
    Down,
}

impl DisplayWindow {
    pub fn new(width: i32, height: i32) -> Self {
        let display = SimulatorDisplay::<Rgb888>::new(Size::new(width as u32, height as u32));
        let output_settings = OutputSettingsBuilder::new().build();
        let window = Window::new("Matrix", &output_settings);
        Self {
            fb: FrameBuffer::new(width, height),
            display,
            window,
        }
    }

    pub fn update(&mut self) -> Event {
        let rect = self.display.bounding_box();
        self.display
            .fill_contiguous(
                &rect,
                self.fb
                    .get_data()
                    .iter()
                    .map(|c| Rgb888::new(c.r, c.g, c.b))
                    .collect::<Vec<Rgb888>>(),
            )
            .map_err(drop)
            .unwrap();
        self.window.update(&self.display);

        for e in self.window.events() {
            match e {
                embedded_graphics_simulator::SimulatorEvent::Quit => return Event::Exit,
                embedded_graphics_simulator::SimulatorEvent::KeyDown { keycode, .. } => {
                    println!("press key: {:?}", keycode);
                    match keycode {
                        Keycode::W => return Event::Go,
                        Keycode::S => return Event::Back,
                        Keycode::A => return Event::TurnLeft,
                        Keycode::D => return Event::TurnRight,
                        Keycode::Q => return Event::Up,
                        Keycode::Z => return Event::Down,
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        Event::Nothing
    }
}
