//! An example of generating a screen with random patterns
use std::{thread::sleep, time::Duration};

use rand::{thread_rng, Rng};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};
use simulate_lcd::{Bitmap, LcdScreen};

fn random_bitmap<const C: usize, const R: usize>() -> Box<Bitmap<C, R>> {
    let mut rng = thread_rng();

    let bits_vec: Vec<[bool; C]> = (0..R as i32)
        .map(|_y| {
            let mut row = [false; C];
            rng.fill(&mut row);
            row
        })
        .collect();

    bits_vec.try_into().unwrap()
}

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
                // Quit
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        screen.draw_bitmap(random_bitmap().as_ref()).unwrap();

        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
