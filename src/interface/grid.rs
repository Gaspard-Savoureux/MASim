use std::{
    ops::{Index, IndexMut},
    vec,
};

use macroquad::{
    color::Color,
    math::{vec2, IVec2, Vec2},
    shapes::{draw_circle, draw_line},
};

#[derive(Debug, Clone, PartialEq)]
pub enum CellState {
    Empty,
    Obstacle,
    Agent,
}

pub struct GridSize {
    pub width: usize,
    pub heigth: usize,
}

pub struct Line {
    src: Vec2,
    dst: Vec2,
}

pub struct Grid {
    /// Coordinate that correspond to the upper-left corner of the grid
    origin: Vec2,
    /// The size of the grid. A
    pub size: GridSize,
    /// The size of a cell
    cell_size: f32,
    /// The lines composing the grid. Stored in the struct to not have to calculate each time
    lines: Vec<Line>,
    /// 2-dimensional array with the state of the cells **LIKELY TO CHANGE**
    cells: Vec<Vec<CellState>>, // NOTE POSSIBLY OBSOLETE
}

impl Grid {
    pub fn new(origin: Vec2, size: GridSize, cell_size: Option<f32>) -> Grid {
        let GridSize { width, heigth } = size;
        let cell_size = cell_size.unwrap_or(16.);

        let mut grid = Grid {
            origin,
            size,
            cell_size,
            lines: Vec::with_capacity(width + heigth),
            cells: vec![vec![CellState::Empty; width]; heigth],
        };

        grid.update_lines(origin, cell_size);

        grid
    }

    fn update_lines(&mut self, origin: Vec2, cell_size: f32) {
        let Vec2 { mut x, mut y } = origin;
        let origin_y: f32 = y;

        let (x_end, y_end) = (
            x + cell_size * self.size.width as f32,
            y + cell_size * self.size.heigth as f32,
        );

        let mut new_lines: Vec<Line> = Vec::with_capacity(self.size.width + self.size.heigth);

        // Lines for the rows
        for _ in 0..self.size.heigth + 1 {
            new_lines.push(Line {
                src: vec2(x, y),
                dst: vec2(x_end, y),
            });
            y += cell_size;
        }

        // reset y origin for the columns
        y = origin_y;

        // Lines for the columns
        for _ in 0..self.size.width + 1 {
            new_lines.push(Line {
                src: vec2(x, y),
                dst: vec2(x, y_end),
            });
            x += cell_size;
        }

        self.lines = new_lines;
    }

    /// Display the grid
    ///
    /// **origin:** represents upper left corner of the grid
    ///
    /// **cell_size:** example: if 16 is given, a cell will have a size of 16x16
    ///
    /// **grid_color:** the color of the line making up the grid
    ///
    /// **agent_positions:** the position of the agents with their colors
    pub fn display(
        &mut self,
        origin: Vec2,
        cell_size: f32,
        grid_color: Color,
        agent_positions: Vec<(IVec2, Color)>,
    ) {
        // IF ORIGIN DIFFERENT UPDATE LINES
        if self.origin.eq(&origin) || self.cell_size != cell_size {
            self.update_lines(origin, cell_size);
        }

        for line in &self.lines {
            draw_line(
                line.src.x, line.src.y, line.dst.x, line.dst.y, 2., grid_color,
            );
        }

        let agent_size = self.cell_size / 2.;

        for (position, color) in agent_positions {
            let IVec2 { x, y } = position;
            draw_circle(
                self.origin.x + (x as f32 * self.cell_size) + agent_size,
                self.origin.y + (y as f32 * self.cell_size) + agent_size,
                agent_size,
                color,
            );
        }
    }
}

// This is simply to implement index on the grid like so: grid[0]
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
        let grid = Grid::new(vec2(10., 10.), GridSize { width, heigth }, None);

        assert_eq!(grid.size.width, width);
        assert_eq!(grid.size.heigth, heigth);
        assert_eq!(grid.cells.len(), heigth);
        for i in 0..grid.size.heigth {
            // heigth
            assert_eq!(grid[&i].len(), width);
            for j in 0..grid.size.width {
                // width
                assert_eq!(grid[&i][j], CellState::Empty);
            }
        }
    }
}
