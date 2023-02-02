//! An example of generating a blank screen
use simulate_lcd::{LcdScreen, Bitmap, LCD_DARK_GREEN, LCD_LIGHT_GREEN};
use sdl2::{event::Event, keyboard::Keycode};

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut screen = LcdScreen::<50, 50>::new(&sdl_context, "LCD Test", 
    LCD_DARK_GREEN, LCD_LIGHT_GREEN, 10, 10).unwrap();
  
    let bm: Box<Bitmap<50, 50>> = vec![[false; 50]; 50].try_into().unwrap();
  
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
  
      screen.draw_bitmap(bm.as_ref()).unwrap();
    }
}