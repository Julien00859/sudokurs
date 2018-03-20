extern crate rand;

use std::fmt;
use std::str;
use rand::Rng;

fn main() {
    let sudoku = Sudoku::new();
    println!("{}", sudoku);
}

fn rotate_1(array: &mut[u8]) {
    let head = array[0].clone();
    for i in 1..9 {
        array[i - 1] = array[i].clone();
    }
    array[8] = head.clone();
}

fn rotate_3(array: &mut[u8]) {
    let mut head = [0u8; 3];
    head.copy_from_slice(&array[..3]);
    for i in 3..9 {
        array[i - 3] = array[i].clone();
    }
    for i in 0..3 {
        array[6 + i] = head[i].clone();
    }
}

struct Sudoku {
    grid: [u8; 81]
}

impl Sudoku {
    pub fn new() -> Sudoku {
        let mut sudoku = Sudoku { grid: [0; 81] };
        sudoku.fill();
        sudoku.shuffle_rows();
        sudoku.transpose();
        sudoku.shuffle_rows();
        sudoku
    }

    fn fill(&mut self) {
        let mut seed : [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        for i in 0..3 {
            for j in 0..3 {
                let start = (i * 27 + j * 9) as usize;
                let end = (start + 9) as usize;
                self.grid[start..end].copy_from_slice(&seed);
                rotate_3(&mut seed);
            }
            rotate_1(&mut seed);
        }
    }

    fn transpose(&mut self) {
        let mut shadow = [0u8; 81];
        shadow.copy_from_slice(&self.grid);
        for i in 0..9 {
            for j in 0..9 {
                self.grid[i * 9 + j] = shadow[j * 9 + i].clone();
            }
        }
    }

    fn shuffle_rows(&mut self) {
        let mut rng = rand::thread_rng();
        let mut from = [0u8; 9];
        let mut to = [0u8; 9];
        for i in 0..3 {
            for swap_from in 0..3 {
                let swap_to = (rng.gen::<u8>() & 0b11) % 3;
                if swap_from == swap_to { continue; }
                let from_idx_start = (i * 27 + swap_from * 9) as usize;
                let from_idx_end = (from_idx_start + 9) as usize;
                let to_idx_start = (i * 27 + swap_to * 9) as usize;
                let to_idx_end = (to_idx_start + 9) as usize;
                from.copy_from_slice(&self.grid[from_idx_start..from_idx_end]);
                to.copy_from_slice(&self.grid[to_idx_start..to_idx_end]);
                self.grid[to_idx_start..to_idx_end].copy_from_slice(&from);
                self.grid[from_idx_start..from_idx_end].copy_from_slice(&to);
            }
        }
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = [0; 161];
        let mut k = 0usize;
        for i in 0..8 {
            for j in 0..8 {
                output[k] = ('0' as u8) + self.grid[i * 9 + j];
                k += 1;
                output[k] = ' ' as u8;
                k += 1;
            }
            output[k] = ('0' as u8) + self.grid[i * 9 + 8];
            k += 1;
            output[k] = '\n' as u8;
            k += 1;
        }
        for i in 0..8 {
            output[k] = ('0' as u8) + self.grid[72 + i];
            k += 1;
            output[k] = ' ' as u8;
            k += 1;
        }
        output[k] = ('0' as u8) + self.grid[80];
        write!(f, "{}", std::str::from_utf8(&output[..]).unwrap())
    }
}
