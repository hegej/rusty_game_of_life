use std::fmt;
use rand::Rng;

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Alive,
    Dead,
}

struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        let mut cells = vec![Cell::Dead; width * height];
        let mut rng = rand::thread_rng();
        
        for cell in cells.iter_mut() {
            *cell = if rng.gen_bool(0.3) { Cell::Alive } else { Cell::Dead };
        }

        Grid { width, height, cells }
    }

    fn get_cell(&self, x: usize, y: usize) -> Cell {
        self.cells[y * self.width + x]
    }

    fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        self.cells[y * self.width + x] = cell;
    }

    fn count_neighbors(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }
                let nx = x as isize + i;
                let ny = y as isize + j;
                if nx >= 0 && ny >= 0 && (nx as usize) < self.width && (ny as usize) < self.height {
                    if self.get_cell(nx as usize, ny as usize) == Cell::Alive {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    fn next_generation(&mut self) {
        let mut new_cells = self.cells.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.get_cell(x, y);
                let live_neighbors = self.count_neighbors(x, y);

                let new_cell = match (cell, live_neighbors) {
                    (Cell::Alive, 2..=3) => Cell::Alive,
                    (Cell::Dead, 3) => Cell::Alive,
                    _ => Cell::Dead,
                };
                new_cells[y * self.width + x] = new_cell;
            }
        }
        self.cells = new_cells;
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let symbol = match self.get_cell(x, y) {
                    Cell::Alive => '◼',
                    Cell::Dead => ' ',
                };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn main() {
    let mut grid = Grid::new(60, 40);

    for _ in 0..100 {
        clear_screen();
        println!("{}", grid);
        grid.next_generation();
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
