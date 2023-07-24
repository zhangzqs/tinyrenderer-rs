use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::{Dimensions, DrawTarget, Size}, image::Image,
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

use crate::draw_target::FrameBuffer;

pub fn show_frame_buffer(fb: FrameBuffer) {
    let mut display =
        SimulatorDisplay::<Rgb888>::new(Size::new(fb.get_width() as u32, fb.get_height() as u32));
    let output_settings = OutputSettingsBuilder::new().build();
    let rect = display.bounding_box();
    display
        .fill_contiguous(
            &rect,
            fb.get_data()
                .iter()
                .map(|c| Rgb888::new(c.r, c.g, c.b))
                .collect::<Vec<Rgb888>>(),
        )
        .map_err(drop)
        .unwrap();
    Window::new("Matrix", &output_settings).show_static(&display);
}
