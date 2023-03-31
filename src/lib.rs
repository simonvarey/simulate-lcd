// * Simulate LCD: A Simple LCD Screen Simulator *

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

#![warn(missing_docs)]
//#![warn(rustdoc::missing_doc_code_examples)]
#![doc = include_str!("../README.md")]

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

/// A [`sdl2::pixels::Color`] object representing the 'on' color of green backlight LCD screens.
pub const LCD_DARK_GREEN: Color = Color::RGB(69, 75, 59);

/// A [`sdl2::pixels::Color`] object representing the 'off' color of green backlight LCD screens.
pub const LCD_LIGHT_GREEN: Color = Color::RGB(158, 171, 136);

// Error

/// Errors that can arise from the creation and use of [`LcdScreens`].
///
/// [`LcdScreens`]: crate::LcdScreen
#[derive(Debug)]
pub enum LcdError {
    /// Indicates that an error occurred when attempting to initalize the SDL video subsystem. This error
    /// is a simple wrapper around the underlying SDL error. Please consult the [`sdl2`] documentation for
    /// more details.
    Video(String),
    /// Indicates that an error occurred when attempting to build the OS window for the [`LcdScreen`]. This
    /// error is a simple wrapper around the [underlying SDL error](https://rust-sdl2.github.io/rust-sdl2/sdl2/video/enum.WindowBuildError.html).
    /// Please consult the [`sdl2`] documentation for more details.
    WindowBuild(WindowBuildError),
    /// Indicates that an error occurred when attempting to build the canvas for the [`LcdScreen`]. This
    /// error is a simple wrapper around the [underlying SDL error](https://rust-sdl2.github.io/rust-sdl2/sdl2/enum.IntegerOrSdlError.html).
    /// Please consult the [`sdl2`] documentation for more details.
    CanvasBuild(IntegerOrSdlError),
    /// Indicates that an error occurred when attempting to fill a dot on the [`LcdScreen`]. This
    /// error is a simple wrapper around the underlying SDL error. Please consult the [`sdl2`] documentation for
    /// more details.
    Fill(String),
    /// Indicates that the [`LcdScreen`] is too wide to be displayed. The maximum width of a screen is [`i32::MAX`]
    /// pixels. As the width of the screen is set by the number of rows of dots it has multiplied by the
    /// pixel width of each dot, one or both of those values must be reduced.
    ///
    /// [`i32::MAX`]: std::i32::MAX
    WindowWidth {
        /// the total width in pixels of the undisplayed screen
        width: u32,
        /// the number of rows of dots of the undisplayed screen
        row: usize,
        /// the pixel width of the dots of the undisplayed screen
        dot_width: u32,
    },
    /// Indicates that the [`LcdScreen`] is too high to be displayed. The maximum height of a screen is [`i32::MAX`]
    /// pixels. As the height of the screen is set by the number of columns of dots it has multiplied by the
    /// height of each dot, one or both of those values must be reduced.
    ///
    /// [`i32::MAX`]: std::i32::MAX
    WindowHeight {
        /// the total height in pixels of the undisplayed screen
        height: u32,
        /// the number of columns of dots of the undisplayed screen
        col: usize,
        /// the pixel height of the dots of the undisplayed screen
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

/// This is an alias for a C-by-R row-major array-of-arrays of booleans. Arrays of this form can be
/// written to an [`LcdScreen`] using the [`draw_bitmap`] method. This alias can be used as a convenience
/// to generate the bitmaps you want to draw to the LCD screen.
///  
/// [`draw_bitmap`]: crate::LcdScreen::draw_bitmap
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

///
/// A simulated LCD dot-matrix screen.
///
/// The screen has `R` rows and `C` columns of dots. *Note*: The number of rows and columns of dots for
/// the screen is specified as a const parameter on the type of the screen, rather than as an argument to
/// the constructor function [`new`].
///
/// # Parameters
///
/// * `R` - The number of rows of dots of the screen
/// * `C` - The number of columns of dots of the screen
///
/// # Examples
///
/// ```
/// use std::{thread::sleep, time::Duration};
///
/// use rand::{thread_rng, Rng};
/// use sdl2::{event::Event, keyboard::Keycode};
/// use simulate_lcd::{Bitmap, LcdScreen, LCD_DARK_GREEN, LCD_LIGHT_GREEN};
///
/// const NANOS_PER_SEC: u64 = 1_000_000_000;
///
/// fn main() {
///     let sdl_context = sdl2::init().unwrap();
///     let mut screen = LcdScreen::<64, 96>::new(
///         &sdl_context,
///         "LCD Example: Random",
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
///          sleep(Duration::from_nanos(NANOS_PER_SEC / 60));
///      }
///  }
/// ```
///
/// [`new`]: crate::LcdScreen::new
pub struct LcdScreen<const R: usize, const C: usize> {
    dots: Box<[[LcdDot; C]; R]>,
    canvas: Canvas<Window>,
    on_color: Color,
    off_color: Color,
}

impl<const R: usize, const C: usize> LcdScreen<R, C> {
    /// Creates a simulated LCD screen.
    ///
    /// *Note*: The number of rows and columns of dots for a screen is specified as a const parameter
    /// on the type of the screen, rather than as an argument to this function.
    ///
    /// # Arguments
    ///
    /// * `sdl_context` - An [`Sdl`] context object
    /// * `title` - The title of the window containing the screen
    /// * `on_color` - A [`Color`] object representing the color of a dot when it is 'on'
    /// * `off_color` - A [`Color`] object representing the color of a dot when it is 'off'
    /// * `dot_width` - The width of a dot on the screen in pixels
    /// * `dot_height` - The height of a dot on the screen in pixels
    ///
    /// # Examples
    ///
    /// ```
    /// # use simulate_lcd::{LcdScreen, LCD_DARK_GREEN, LCD_LIGHT_GREEN};
    /// # let sdl_context = sdl2::init().unwrap();
    /// let mut screen = LcdScreen::<64, 96>::new(
    ///         &sdl_context,
    ///         "LCD Example: Blank",
    ///         LCD_DARK_GREEN,
    ///         LCD_LIGHT_GREEN,
    ///         10,
    ///         10,
    ///      )
    ///      .unwrap();
    /// # std::thread::sleep(std::time::Duration::from_secs(1));
    /// ```
    ///
    /// # Errors
    ///
    /// - [`LcdError::Video`] when there is an error initializing the SDL video subsystem
    /// - [`LcdError::WindowBuild`] when there is an error building the window
    /// - [`LcdError::CanvasBuild`] when there is an error building the window canvas
    /// - [`LcdError::WindowWidth`] when the total window width, in pixels, would exceed [`i32::MAX`]
    /// - [`LcdError::WindowHeight`] when the total window width, in pixels, would exceed [`i32::MAX`]
    ///
    /// [`Sdl`]: sdl2::Sdl
    /// [`Color`]: sdl2::pixels::Color
    /// [`i32::MAX`]: std::i32::MAX
    ///
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

        let video_subsystem = sdl_context.video().map_err(LcdError::Video)?;

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

    /// Draws a bitmap to a simulated LCD screen.
    ///
    /// # Arguments
    ///
    /// * `bm` - A [`Bitmap`], or something that can be converted into a bitmap, to write to the LCD screen
    ///
    /// # Examples
    ///
    /// ```
    /// # use simulate_lcd::{LcdScreen, LCD_DARK_GREEN, LCD_LIGHT_GREEN};
    /// # let sdl_context = sdl2::init().unwrap();
    /// let mut screen = LcdScreen::<2, 2>::new(
    ///         &sdl_context,
    ///         "LCD Example: Checkerboard",
    ///         LCD_DARK_GREEN,
    ///         LCD_LIGHT_GREEN,
    ///         100,
    ///         100,
    ///      )
    ///      .unwrap();
    /// # std::thread::sleep(std::time::Duration::from_secs(1));
    ///
    /// screen.draw_bitmap(&[[true, false], [false, true]]);
    /// # std::thread::sleep(std::time::Duration::from_secs(1));
    /// ```
    ///
    /// # Errors
    ///
    /// - [`LcdError::Fill`] when there is an error filling one of the dots with the relevant color
    ///
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
                    self.canvas.fill_rect(dot.rect).map_err(LcdError::Fill)?;
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
