# simulate_lcd <!-- [![](https://img.shields.io/crates/v/simulate_lcd.svg)](https://crates.io/crates/simulate_lcd) --> <!-- [![](https://docs.rs/simulate_lcd/badge.svg)](https://docs.rs/simulate_lcd) -->

A simple library to simulate dot-matrix displays, such as monochrome LCD screens.

<!-- [Documentation](https://docs.rs/simulate_lcd) -->

<!-- ## Overview -->

## Example <!-- 'Example Usage' -->

<!-- ``` ``` -->

More examples can be found in the examples folder.

## Usage

To create new simulated screen window with `R` rows and `C` columns of dots, use the function `LcdScreen::<R, C>::new`, with the following parameters:

- `sdl_context`: an SDL context object from the [sdl2](https://crates.io/crates/sdl2)
- `title`: the window title
- `on_color`: the color of a dot when it is 'on'. e.g. (near) black on a backlight LCD screen
- `off_color`: the color of a dot when it is 'off'. e.g. green on a green backlight LCD screen
- `dot_width`: the width of a dot in pixels of the actual window
- `dot_height`: the height of a dot in pixels of the actual window

New images can be drawn to the screen using the `draw_bitmap` method.

## Setup

<!-- requires -->
simulate_lcd is built around the [sdl2](https://crates.io/crates/sdl2) crate. Note that sdl2 may require future setup than just adding the crate. See the [sdl2 README](https://github.com/Rust-SDL2/rust-sdl2/blob/master/README.md#requirements) for details.

## Examples



<!-- 
License

Licensed under either of

    Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
    MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT) at your option.

Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be dual licensed as above, without any additional terms or conditions.
Dependencies

~0–6.5MB
~105K SLoC

    unix nix 0.26+fs+signal
    windows windows-sys 0.42+Win32_Foundation+Win32_System_Threadi…+Win32_Security+Win32_System_Windows…+Win32_System_Console


-->