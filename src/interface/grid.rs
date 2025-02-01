use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, PartialEq)]
pub enum CellState {
    Empty,
    Obstacle,
    Agent,
}

// struct Cell {
//     pub color: Color,
//     pub state: CellState,
//     // pub agent_type // Not sure about the implementation yet
// }

pub struct Grid {
    /// (width, heigth)
    size: (usize, usize),
    /// 2-dimensional array with the state of the cells **LIKELY TO CHANGE**
    cells: Vec<Vec<CellState>>,
}

impl Grid {
    pub fn new(size: (usize, usize)) -> Grid {
        let (width, heigth) = size;
        Grid {
            size,
            cells: vec![vec![CellState::Empty; width]; heigth],
        }
    }
}

impl Index<&'_ usize> for Grid {
    type Output = Vec<CellState>;
    fn index(&self, index: &usize) -> &Self::Output {
        &self.cells[*index]
    }
}

impl IndexMut<&'_ usize> for Grid {
    fn index_mut(&mut self, index: &'_ usize) -> &mut Self::Output {
        &mut self.cells[*index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_creation() {
        let (width, heigth) = (2 as usize, 3 as usize);
        let grid = Grid::new((width, heigth));

        assert_eq!(grid.size.0, width);
        assert_eq!(grid.size.1, heigth);
        assert_eq!(grid.cells.len(), heigth);
        for i in 0..grid.size.1 {
            // heigth
            assert_eq!(grid[&i].len(), width);
            for j in 0..grid.size.0 {
                // width
                assert_eq!(grid[&i][j], CellState::Empty);
            }
        }
    }
}
