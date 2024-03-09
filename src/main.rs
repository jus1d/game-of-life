const ROWS: usize = 10;
const COLS: usize = 20;

#[derive(Clone, Copy, Debug)]
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
        for row in &self.cells {
            for &cell in row {
                match cell {
                    Cell::Dead => print!("."),
                    Cell::Alive => print!("#"),
                }
            }
            println!();
        }
    }
}

fn main() {
    let mut field = Field::new();
    field.display();
}
