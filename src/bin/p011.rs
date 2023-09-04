use std::slice::Iter;

const CELLS: [[u8; 20]; 20] = [
    [08,02,22,97,38,15,00,40,00,75,04,05,07,78,52,12,50,77,91,08],
    [49,49,99,40,17,81,18,57,60,87,17,40,98,43,69,48,04,56,62,00],
    [81,49,31,73,55,79,14,29,93,71,40,67,53,88,30,03,49,13,36,65],
    [52,70,95,23,04,60,11,42,69,24,68,56,01,32,56,71,37,02,36,91],
    [22,31,16,71,51,67,63,89,41,92,36,54,22,40,40,28,66,33,13,80],
    [24,47,32,60,99,03,45,02,44,75,33,53,78,36,84,20,35,17,12,50],
    [32,98,81,28,64,23,67,10,26,38,40,67,59,54,70,66,18,38,64,70],
    [67,26,20,68,02,62,12,20,95,63,94,39,63,08,40,91,66,49,94,21],
    [24,55,58,05,66,73,99,26,97,17,78,78,96,83,14,88,34,89,63,72],
    [21,36,23,09,75,00,76,44,20,45,35,14,00,61,33,97,34,31,33,95],
    [78,17,53,28,22,75,31,67,15,94,03,80,04,62,16,14,09,53,56,92],
    [16,39,05,42,96,35,31,47,55,58,88,24,00,17,54,24,36,29,85,57],
    [86,56,00,48,35,71,89,07,05,44,44,37,44,60,21,58,51,54,17,58],
    [19,80,81,68,05,94,47,69,28,73,92,13,86,52,17,77,04,89,55,40],
    [04,52,08,83,97,35,99,16,07,97,57,32,16,26,26,79,33,27,98,66],
    [88,36,68,87,57,62,20,72,03,46,33,67,46,55,12,32,63,93,53,69],
    [04,42,16,73,38,25,39,11,24,94,72,18,08,46,29,32,40,62,76,36],
    [20,69,36,41,72,30,23,88,34,62,99,69,82,67,59,85,74,04,36,16],
    [20,73,35,29,78,31,90,01,74,31,49,71,48,86,81,16,23,57,05,54],
    [01,70,54,71,83,51,54,69,16,92,33,48,61,43,52,01,89,19,67,48],
];

enum Direction {
    North,
    NorthWest,
    West,
    SouthWest,
    South,
    SouthEast,
    East,
    NorthEast,
}

impl Direction {
    fn iter() -> Iter<'static, Direction> {
        static DIRS: [Direction; 8] = [
            Direction::North,
            Direction::NorthWest,
            Direction::West,
            Direction::SouthWest,
            Direction::South,
            Direction::SouthEast,
            Direction::East,
            Direction::NorthEast,
        ];
        DIRS.iter()
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Cell {
    x: usize,
    y: usize,
    value: u8,
}

struct Grid {
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    fn new() -> Self {
        let cells = CELLS.iter().enumerate().map(|(y, row)| {
            row.iter().enumerate().map(|(x, value)| Cell { x, y, value: *value }).collect()
        }).collect();

        Grid { cells }
    }

    fn get_cell(&self, x: usize, y: usize) -> Option<&Cell> {
        if x >= 20 || y >= 20 {
            return None;
        }

        Some(&self.cells[y][x])
    }

    fn get_adjacent_cell(&self, cell: &Cell, direction: &Direction) -> Option<&Cell> {
        let (dx, dy) = match direction {
            Direction::North => (0, -1),
            Direction::NorthWest => (-1, -1),
            Direction::West => (-1, 0),
            Direction::SouthWest => (-1, 1),
            Direction::South => (0, 1),
            Direction::SouthEast => (1, 1),
            Direction::East => (1, 0),
            Direction::NorthEast => (1, -1),
        };

        let x = (cell.x as i8).checked_add(dx)?;
        let y = (cell.y as i8).checked_add(dy)?;

        self.get_cell(x as usize, y as usize)
    }
}

fn main() {
    println!("{:?}", solve());
}

fn solve() -> u32 {
    let mut highest_product = 0;
    let grid = Grid::new();
    
    grid.cells.iter().for_each(|row| {
        row.iter().for_each(|cell| {
            Direction::iter().for_each(|dir| {
                let mut product = cell.value as u32;

                let mut current_cell = Some(cell);
                for _ in 0..3 {
                    current_cell = grid.get_adjacent_cell(current_cell.unwrap(), dir);

                    if current_cell.is_none() {
                        product = 0;
                        break;
                    }

                    product *= current_cell.unwrap().value as u32;
                }

                if product > highest_product {
                    highest_product = product;
                }
            });
        });
    });

    highest_product
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_grid() {
        let grid = Grid::new();

        assert_eq!(8, grid.get_cell(0,0).unwrap().value);
        assert_eq!(48, grid.get_cell(19,19).unwrap().value);

        assert_eq!(None, grid.get_cell(20, 4));
        assert_eq!(None, grid.get_cell(4, 20));
    }

    #[test]
    fn adjacent_cell() {
        let grid = Grid::new();

        let cell = grid.get_cell(9, 9).unwrap();
        assert_eq!(grid.get_cell(9, 8), grid.get_adjacent_cell(cell, &Direction::North));
        assert_eq!(grid.get_cell(8, 8), grid.get_adjacent_cell(cell, &Direction::NorthWest));
        assert_eq!(grid.get_cell(8, 9), grid.get_adjacent_cell(cell, &Direction::West));
        assert_eq!(grid.get_cell(8, 10), grid.get_adjacent_cell(cell, &Direction::SouthWest));
        assert_eq!(grid.get_cell(9, 10), grid.get_adjacent_cell(cell, &Direction::South));
        assert_eq!(grid.get_cell(10, 10), grid.get_adjacent_cell(cell, &Direction::SouthEast));
        assert_eq!(grid.get_cell(10, 9), grid.get_adjacent_cell(cell, &Direction::East));
        assert_eq!(grid.get_cell(10, 8), grid.get_adjacent_cell(cell, &Direction::NorthEast));
    }
}
