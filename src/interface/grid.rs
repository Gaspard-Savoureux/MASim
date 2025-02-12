use macroquad::{
    color::Color,
    math::{vec2, IVec2, Vec2},
    shapes::{draw_circle, draw_line, draw_rectangle},
};

use crate::scheduler::scheduler::Position;

// #[derive(Debug, Clone, PartialEq)]
// pub enum CellState {
//     Empty,
//     Obstacle,
//     Agent,
// }

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
    start: Vec2,
    /// Coordinate that correspond to the lower-right corner of the grid
    end: Vec2,
    /// The size of the grid.
    pub size: GridSize,
    /// The lines composing the grid. Stored in the struct to not have to calculate each time
    lines: Vec<Line>,
    /// Element with persistent long term position such as obstacles (walls, bushes, etc.), the goal cell, etc.
    persistent_elements: Vec<(Position, Color)>,
}

impl Grid {
    pub fn new(
        start: Vec2,
        end: Vec2,
        size: GridSize,
        persistent_elements: Vec<(Position, Color)>,
    ) -> Grid {
        let GridSize { width, heigth } = size;

        let mut grid = Grid {
            start,
            end,
            size,
            lines: Vec::with_capacity(width + heigth),
            // cells: vec![vec![CellState::Empty; width]; heigth],
            persistent_elements,
        };

        grid.update_lines(start, end);

        grid
    }

    pub fn update_persistent_element(&mut self, new_elements: Vec<(Position, Color)>) {
        self.persistent_elements = new_elements;
    }

    fn update_lines(&mut self, start: Vec2, end: Vec2) {
        let Vec2 { mut x, mut y } = start;
        let Vec2 { x: x_end, y: y_end } = end;

        let origin_y: f32 = y;

        let (cell_width, cell_heigth) = (
            (x_end - x) / self.size.width as f32,
            (y_end - y) / self.size.heigth as f32,
        );

        let mut new_lines: Vec<Line> = Vec::with_capacity(self.size.width + self.size.heigth);

        // Lines for the rows
        for _ in 0..self.size.heigth + 1 {
            new_lines.push(Line {
                src: vec2(x, y),
                dst: vec2(x_end, y),
            });
            y += cell_heigth;
        }

        // reset y origin for the columns
        y = origin_y;

        // Lines for the columns
        for _ in 0..self.size.width + 1 {
            new_lines.push(Line {
                src: vec2(x, y),
                dst: vec2(x, y_end),
            });
            x += cell_width;
            // x += cell_size;
        }

        self.lines = new_lines;
        self.start = start;
        self.end = end;
    }

    /// Display the grid
    ///
    /// **start:** represents upper left corner of the grid
    ///
    /// **end:** represents lower right corner of the grid
    ///
    /// **cell_size:** example: if 16 is given, a cell will have a size of 16x16
    ///
    /// **grid_color:** the color of the line making up the grid
    ///
    /// **agent_positions:** the position of the agents with their colors
    ///
    /// NOTE: Some line appear thicker from time to time
    pub fn display(
        &mut self,
        start: Vec2,
        end: Vec2,
        grid_color: Color,
        agent_positions: Vec<(IVec2, Color)>,
    ) {
        // IF ORIGIN DIFFERENT UPDATE LINES
        if !self.start.eq(&start) || !self.end.eq(&end) {
            self.update_lines(start, end);
        }

        for line in &self.lines {
            draw_line(
                line.src.x, line.src.y, line.dst.x, line.dst.y, 1., grid_color,
            );
        }

        let Vec2 {
            x: x_start,
            y: y_start,
        } = start;
        let Vec2 { x: x_end, y: y_end } = end;
        let (cell_width, cell_heigth) = (
            (x_end - x_start) / self.size.width as f32,
            (y_end - y_start) / self.size.heigth as f32,
        );

        // let agent_size = cell_width / 2.;
        let agent_size = cell_heigth / 2. - 4.;
        // Draw agents
        for (position, color) in agent_positions {
            let IVec2 { x, y } = position;
            draw_circle(
                x_start + (x as f32 * cell_width) + cell_width / 2.,
                y_start + (y as f32 * cell_heigth) + cell_heigth / 2.,
                agent_size,
                color,
            );
        }

        // Draw persitent elements
        for (position, color) in &self.persistent_elements {
            let IVec2 { x, y } = position;
            draw_rectangle(
                x_start + (*x as f32 * cell_width),
                y_start + (*y as f32 * cell_heigth),
                cell_width,
                cell_heigth,
                *color,
            );
        }
    }
}

// This is simply to implement index on the grid like so: grid[0]
// impl Index<&'_ usize> for Grid {
//     type Output = Vec<CellState>;
//     fn index(&self, index: &usize) -> &Self::Output {
//         &self.cells[*index]
//     }
// }

// impl IndexMut<&'_ usize> for Grid {
//     fn index_mut(&mut self, index: &'_ usize) -> &mut Self::Output {
//         &mut self.cells[*index]
//     }
// }

// #[cfg(test)]
// mod tests {

//     #[test]
//     fn grid_creation() {
//         // let (width, heigth) = (2 as usize, 3 as usize);
//         // let grid = Grid::new(
//         //     vec2(10., 10.),
//         //     vec2(20., 20.),
//         //     GridSize { width, heigth },
//         //     None,
//         // );

//         // assert_eq!(grid.size.width, width);
//         // assert_eq!(grid.size.heigth, heigth);
//         // assert_eq!(grid.cells.len(), heigth);
//         // for i in 0..grid.size.heigth {
//         //     // heigth
//         //     assert_eq!(grid[&i].len(), width);
//         //     for j in 0..grid.size.width {
//         //         // width
//         //         assert_eq!(grid[&i][j], CellState::Empty);
//         //     }
//         // }
//     }
// }
