// * Simulate LCD: A Simple LCD Screen Simulator *
// Created by Simon Varey - github.com/simonvarey

// Imports

use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

use sdl2::{
    pixels::Color,
    rect::Rect,
    render::Canvas,
    video::{Window, WindowBuildError},
    IntegerOrSdlError, Sdl,
};

// Constants

pub const LCD_DARK_GREEN: Color = Color::RGB(69, 75, 59);
pub const LCD_LIGHT_GREEN: Color = Color::RGB(158, 171, 136);

// Error

#[derive(Debug)]
pub enum LcdError {
    Video(String),
    WindowBuild(WindowBuildError),
    CanvasBuild(IntegerOrSdlError),
    Fill(String),
    WindowWidth {
        width: u32,
        row: usize,
        dot_width: u32,
    },
    WindowHeight {
        height: u32,
        col: usize,
        dot_height: u32,
    },
}

impl Display for LcdError {
    fn fmt(&self, fmtr: &mut Formatter<'_>) -> FmtResult {
        match self {
            LcdError::Video(err) => write!(fmtr, "Error initalizing video subsystem: {err}"),
            LcdError::WindowBuild(err) => write!(fmtr, "Error building window: {err}"),
            LcdError::CanvasBuild(err) => write!(fmtr, "Error building canvas: {err}"),
            LcdError::Fill(err) => write!(fmtr, "Error filling dot: {err}"),
            LcdError::WindowWidth { width, row, dot_width }
                => write!(fmtr, "{width} pixels is too large for a window width. Window width cannot be larger than {}. Reduce either the number of dot rows {row} or the width {dot_width} of dots.", i32::MAX),
            LcdError::WindowHeight { height, col, dot_height }
                => write!(fmtr, "{height} pixels is too large for a window height. Window height cannot be larger than {}. Reduce either the number of dot columns {col} or the height {dot_height} of dots.", i32::MAX),
        }
    }
}

impl Error for LcdError {}

impl From<WindowBuildError> for LcdError {
    fn from(err: WindowBuildError) -> Self {
        Self::WindowBuild(err)
    }
}

impl From<IntegerOrSdlError> for LcdError {
    fn from(err: IntegerOrSdlError) -> Self {
        Self::CanvasBuild(err)
    }
}

// Bitmap

pub type Bitmap<const C: usize, const R: usize> = [[bool; C]; R];

// LCD Dot

#[derive(Debug)]
struct LcdDot {
    rect: Rect,
    on: bool,
}

impl LcdDot {
    fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        assert!((1..=(i32::MAX as u32)).contains(&width), "INTERNAL ERROR: the width of a TIDot must be > 0 and <= i32::MAX. If you are seeing this error then RusTI-BASIC has a bug.");
        assert!((1..=(i32::MAX as u32)).contains(&height), "INTERNAL ERROR: the height of a TIDot must be > 0 and <= i32::MAX. If you are seeing this error then RusTI-BASIC has a bug.");

        Self {
            rect: Rect::new(
                x * width as i32, // Note: as this has been checked to be positive, this is a true cast
                y * height as i32, // Note: as this has been checked to be positive, this is a true cast
                width,
                height,
            ),
            on: false,
        }
    }
}

// * LCD Screen *

/// ```
/// use std::{thread::sleep, time::Duration};

/// use rand::{thread_rng, Rng};
/// use sdl2::{event::Event, keyboard::Keycode};
/// use simulate_lcd::{Bitmap, LcdScreen, LCD_DARK_GREEN, LCD_LIGHT_GREEN};

/// fn main() {
///     let sdl_context = sdl2::init().unwrap();
///     let mut screen = LcdScreen::<64, 96>::new(
///         &sdl_context,
///         "LCD Demo: Random",
///         LCD_DARK_GREEN,
///         LCD_LIGHT_GREEN,
///         10,
///         10,
///      )
///      .unwrap();
/// 
///      let mut event_pump = sdl_context.event_pump().unwrap();
///      'running: loop {
///         for event in event_pump.poll_iter() {
///             match event {
///                 Event::Quit { .. }
///                 | Event::KeyDown {
///                     keycode: Some(Keycode::Escape),
///                     ..
///                 } => break 'running,
///                 _ => {}
///              }
///          }
///          let mut rng = thread_rng();
/// 
///          let random_bits: Vec<[bool; 96]> = (0..64).map(|_| rng.gen()).collect();
/// 
///          screen.draw_bitmap(&random_bits.try_into().unwrap()).unwrap();
/// 
///          sleep(Duration::new(0, 1_000_000_000u32 / 60));
///      }
///  }
///```
pub struct LcdScreen<const R: usize, const C: usize> {
    dots: Box<[[LcdDot; C]; R]>,
    canvas: Canvas<Window>,
    on_color: Color,
    off_color: Color,
}

    // Use: LcdScreen::<R, C>::new(...)
impl<const R: usize, const C: usize> LcdScreen<R, C> {
    pub fn new(
        sdl_context: &Sdl,
        title: &str,
        on_color: Color,
        off_color: Color,
        dot_width: u32,
        dot_height: u32,
    ) -> Result<LcdScreen<R, C>, LcdError> {
        // Note: usize can be truly cast to u32.
        let window_width = (C as u32) * dot_width;
        let window_height = (R as u32) * dot_height;

        // Note: if window_width/window_height are between 1 and i32::MAX then both R/C and
        //   dot_width/dot_height must be between 1 and i32::MAX. Also, i32::MAX can be truly cast to u32.
        if !(1..=(i32::MAX as u32)).contains(&window_width) {
            Err(LcdError::WindowWidth {
                width: window_width,
                row: R,
                dot_width,
            })?
        };
        if !(1..=(i32::MAX as u32)).contains(&window_height) {
            Err(LcdError::WindowHeight {
                height: window_height,
                col: C,
                dot_height,
            })?
        };

        // Set up window

        let video_subsystem = sdl_context.video().map_err(|err| LcdError::Video(err))?;

        let window = video_subsystem
            .window(title, window_width, window_height)
            .position_centered()
            .build()?; //TODO: provide more options than just centered

        let mut canvas = window.into_canvas().build()?;

        canvas.set_draw_color(off_color);
        canvas.clear();
        canvas.present();

        // Create screen

        //Note: R and C can be truly cast to i32 as they have been proved to be less than i32::MAX
        let dots_vec: Vec<[LcdDot; C]> = (0..R as i32)
            .map(|y| {
                let row_vec: Vec<LcdDot> = (0..C as i32)
                    .map(|x| LcdDot::new(x, y, dot_width, dot_height))
                    .collect();
                row_vec.try_into().unwrap() // Note: every row_vec must be C in length, so this cannot fail
            })
            .collect();

        // Note: dots_vec must be R in length, so this cannot fail
        Ok(Self {
            dots: dots_vec.try_into().unwrap(),
            canvas,
            on_color,
            off_color,
        })
    }

    pub fn draw_bitmap<'a, BM: Into<&'a Bitmap<C, R>>>(&mut self, bm: BM) -> Result<(), LcdError> {
        let bm_array: &[[bool; C]; R] = bm.into();
        for (row_dots, row_bm) in self.dots.iter_mut().zip(bm_array) {
            for (dot, bit) in row_dots.iter_mut().zip(row_bm) {
                if dot.on != *bit {
                    dot.on = *bit;
                    self.canvas.set_draw_color(if dot.on {
                        self.on_color
                    } else {
                        self.off_color
                    });
                    self.canvas
                        .fill_rect(dot.rect)
                        .map_err(|err| LcdError::Fill(err))?;
                }
            }
        }
        self.canvas.present();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use sdl2::{event::Event, keyboard::Keycode};

    #[test]
    fn test_success() {
        let sdl_context = sdl2::init().unwrap();
        let _screen = LcdScreen::<10, 10>::new(
            &sdl_context,
            "LCD Test: Success",
            LCD_DARK_GREEN,
            LCD_LIGHT_GREEN,
            10,
            10,
        )
        .unwrap();
    }
}
