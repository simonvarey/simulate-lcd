//! An example of generating a screen with random patterns
use std::{thread::sleep, time::Duration};

use simulate_lcd::{LcdScreen, Bitmap, LCD_DARK_GREEN, LCD_LIGHT_GREEN};
use sdl2::{event::Event, keyboard::Keycode};
use rand::{thread_rng, Rng};

fn random_bitmap<const C: usize, const R: usize>() -> Box<Bitmap<C, R>> {
  let mut rng = thread_rng();
  
  let try_bits_vec: Result<Vec<[bool; C]>, Vec<bool>> = (0..R as i32).map(|_y| {
    let row_vec: Vec<bool> = (0..C as i32).map(|_x| rng.gen()).collect();
    row_vec.try_into()
  }).collect();

  let bits_vec = try_bits_vec.unwrap();
  bits_vec.try_into().unwrap()
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut screen = LcdScreen::<60, 40>::new(&sdl_context, "LCD Test: Random", 
    LCD_DARK_GREEN, LCD_LIGHT_GREEN, 10, 10).unwrap();
  
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
      for event in event_pump.poll_iter() {
        match event {
          // Quit
          Event::Quit {..} |
          Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
            break 'running
          },
          _ => { }
        }
      }
  
      screen.draw_bitmap(random_bitmap().as_ref()).unwrap();

      sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
