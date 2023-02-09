//! An example of generating a screen with random patterns
use std::{thread::sleep, time::Duration};

use rand::{thread_rng, Rng};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

use simulate_lcd::LcdScreen;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut screen = LcdScreen::<15, 50>::new(
        &sdl_context,
        "LCD Example: Random",
        Color::WHITE,
        Color::BLACK,
        20,
        35,
    )
    .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        let mut rng = thread_rng();
        let random_bits: Vec<[bool; 50]> = (0..15).map(|_| rng.gen()).collect();
        screen.draw_bitmap(&random_bits.try_into().unwrap()).unwrap();

        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
