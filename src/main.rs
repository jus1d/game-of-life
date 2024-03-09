use std::thread;
use std::time;

const ROWS: usize = 10;
const COLS: usize = 20;
const ALIVE_SYMBOL: char = '@';
const DEAD_SYMBOL: char = '.';
const DELAY: time::Duration = time::Duration::from_millis(500);

#[derive(Clone, Copy, Debug, PartialEq)]
enum Cell {
    Dead,
    Alive,
}

struct Field {
    cells: [[Cell; COLS]; ROWS]
}

impl Field {
    fn new() -> Self {
        Field {
            cells: [[Cell::Dead; COLS]; ROWS],
        }
    }

    fn display(&self) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        for row in &self.cells {
            for &cell in row {
                match cell {
                    Cell::Dead => print!("{}", DEAD_SYMBOL),
                    Cell::Alive => print!("{}", ALIVE_SYMBOL),
                }
            }
            println!();
        }
    }

    fn next_generation(&mut self) {
        let mut next_state = [[Cell::Dead; COLS]; ROWS];

        for row in 0..ROWS {
            for col in 0..COLS {
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

                if neighbor_row >= 0 && neighbor_row < ROWS as isize && neighbor_col >= 0 && neighbor_col < COLS as isize {
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
    let mut field = Field::new();

    field.set_glider();

    loop {
        field.display();
        field.next_generation();
        thread::sleep(DELAY);
    }
}
