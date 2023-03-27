// Copyright 2023 Simon Varey - github.com/simonvarey

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! An example of generating a blank screen
use sdl2::{event::Event, keyboard::Keycode};
use simulate_lcd::{Bitmap, LcdScreen, LCD_DARK_GREEN, LCD_LIGHT_GREEN};

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut screen = LcdScreen::<50, 50>::new(
        &sdl_context,
        "LCD Example: Blank",
        LCD_DARK_GREEN,
        LCD_LIGHT_GREEN,
        10,
        10,
    )
    .unwrap();

    let bm: Box<Bitmap<50, 50>> = vec![[false; 50]; 50].try_into().unwrap();

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

        screen.draw_bitmap(bm.as_ref()).unwrap();
    }
}
