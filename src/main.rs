use std::thread;
use std::time;

const DEAD_CHAR: char = '.';
const ALIVE_CHAR: char = '@';
const DELAY_MS: u64 = 100;

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Dead,
    Alive
}

struct Field {
    cells: Vec<Vec<Cell>>,
    rows: usize,
    cols: usize,
}

impl Field {
    fn new(rows: usize, cols: usize) -> Self {
        let cells = vec![vec![Cell::Dead; cols]; rows];
        Field {
            cells,
            rows,
            cols,
        }
    }

    fn display(&self) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        for row in 0..self.rows {
            for col in 0..self.cols {
                match self.cells[row][col] {
                    Cell::Dead => print!("{}", DEAD_CHAR),
                    Cell::Alive => print!("{}", ALIVE_CHAR),
                }
            }
            println!();
        }
    }

    fn next(&mut self) {
        let mut next_state = vec![vec![Cell::Dead; self.cols]; self.rows];

        for row in 0..self.rows {
            for col in 0..self.cols {
                let neighbors = self.count_alive_neighbors(row, col);
                let mut cell = self.cells[row][col];

                match cell {
                    Cell::Dead => {
                        if neighbors == 3 {
                            cell = Cell::Alive;
                        }
                    },
                    Cell::Alive => {
                        if neighbors < 2 || neighbors > 3 {
                            cell = Cell::Dead;
                        }
                    },
                }
                next_state[row][col] = cell;
            }
        }

        self.cells = next_state;
    }

    fn count_alive_neighbors(&self, row: usize, col: usize) -> u8 {
        let mut count = 0;

        for d_row in -1..=1 {
            for d_col in -1..=1 {
                if d_row == 0 && d_col == 0 {
                    continue;
                }

                let neighbor_row = row as isize + d_row;
                let neighbor_col = col as isize + d_col;

                if neighbor_row >= 0 && neighbor_row < self.rows as isize && neighbor_col >= 0 && neighbor_col < self.cols as isize {
                    let neighbor_row = neighbor_row as usize;
                    let neighbor_col = neighbor_col as usize;

                    if self.cells[neighbor_row][neighbor_col] == Cell::Alive {
                        count += 1;
                    }
                }
            }
        }

        return count;
    }

    fn set_glider(&mut self) {
        self.cells[3][3] = Cell::Alive;
        self.cells[4][4] = Cell::Alive;
        self.cells[4][5] = Cell::Alive;
        self.cells[3][5] = Cell::Alive;
        self.cells[2][5] = Cell::Alive;
    }
}

fn main() {
    let mut field = Field::new(20, 40);
    field.set_glider();

    loop {
        field.next();
        field.display();
        thread::sleep(time::Duration::from_millis(DELAY_MS));
    }
}