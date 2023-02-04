//! An example of generating a screen which plays Conway's Game of Life
use std::{thread::sleep, time::Duration};

use rand::{thread_rng, Rng};
use sdl2::{event::Event, keyboard::Keycode};
use simulate_lcd::{Bitmap, LcdScreen, LCD_DARK_GREEN, LCD_LIGHT_GREEN};

fn random_bitmap<const C: usize, const R: usize>() -> Box<Bitmap<C, R>> {
    let mut rng = thread_rng();

    let try_bits_vec: Result<Vec<[bool; C]>, Vec<bool>> = (0..R as i32)
        .map(|_y| {
            let row_vec: Vec<bool> = (0..C as i32).map(|_x| rng.gen()).collect();
            row_vec.try_into()
        })
        .collect();

    let bits_vec = try_bits_vec.unwrap();
    bits_vec.try_into().unwrap()
}

fn neighbour_count<const C: usize, const R: usize>(
    bm: &Bitmap<C, R>,
    nrow: usize,
    ncol: usize,
) -> usize {
    let mut neighbours = 0;

    for row_offset in -1..=1 {
        for col_offset in -1..=1 {
            let check_row = (nrow as isize) + row_offset;
            let check_col = (ncol as isize) + col_offset;
            if (check_row >= 0) && (check_col >= 0) {
                let check_row = check_row as usize;
                let check_col = check_col as usize;
                // TODO: Change to chained if lets after chained if lets stablize: https://github.com/rust-lang/rust/issues/53667
                if (check_row != nrow) || (check_col != ncol) {
                    if let Some(row) = bm.get(check_row) {
                        if let Some(cell) = row.get(check_col) {
                            neighbours += *cell as usize
                        }
                    }
                }
            }
        }
    }

    neighbours
}

fn next_state<const C: usize, const R: usize>(old: Bitmap<C, R>) -> Bitmap<C, R> {
    let mut new = [[false; C]; R];
    for nrow in 0..R {
        for ncol in 0..C {
            let n_neighbours = neighbour_count(&old, nrow, ncol);
            if old[nrow][ncol] {
                if n_neighbours == 2 || n_neighbours == 3 {
                    new[nrow][ncol] = true
                }
            } else {
                if n_neighbours == 3 {
                    new[nrow][ncol] = true
                }
            }
        }
    }
    new
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut screen = LcdScreen::<65, 120>::new(
        &sdl_context,
        "LCD Test: Game of Life",
        LCD_DARK_GREEN,
        LCD_LIGHT_GREEN,
        10,
        10,
    )
    .unwrap();

    let mut bm = random_bitmap();

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

        bm = Box::new(next_state(*bm));

        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
