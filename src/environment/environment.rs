use macroquad::{
    color::Color,
    math::{IVec2, Vec2},
};

use crate::{
    agent::learning_agent::{Action, Done},
    interface::grid::{Grid, GridSize},
    scheduler::scheduler::{AgentRef, Position},
};

pub struct Env {
    grid: Grid,
    pub actions: Vec<Action>,
    /// TODO change this, only temporary
    pub goal: (i32, i32),
    pub prefered_cells: Vec<(i32, i32)>,
}

impl Env {
    pub fn new(origin: Vec2, size: GridSize, cell_size: Option<f32>, actions: &[Action]) -> Env {
        Env {
            grid: Grid::new(origin, size, cell_size),
            actions: Vec::from(actions),
            goal: (8, 8),
            prefered_cells: vec![
                (7, 7),
                (7, 8),
                (7, 9),
                (8, 7),
                (8, 9),
                (9, 9),
                (9, 8),
                (9, 7),
            ],
        }
    }

    pub fn get_width(&self) -> &usize {
        &self.grid.size.width
    }

    pub fn get_heigth(&self) -> &usize {
        &self.grid.size.heigth
    }

    pub fn display_grid(
        &mut self,
        origin: Vec2,
        cell_size: f32,
        grid_color: Color,
        agent_positions: Vec<(IVec2, Color)>,
    ) {
        self.grid
            .display(origin, cell_size, grid_color, agent_positions);
    }

    pub fn valid_position(&self, position: IVec2) -> bool {
        let IVec2 { x, y } = position;

        x >= 0 && x < self.grid.size.width as i32 && // x
        y >= 0 && y < self.grid.size.heigth as i32 // y
    }

    pub fn step(&self, position: Position, agent: &mut AgentRef) -> (Position, Done) {
        let mut agent = agent.borrow_mut();

        let action = agent.choose_action(&agent.state, &self.actions);

        let (new_position, next_state, reward, done) =
            agent.step(&self, position, &agent.state, &action);

        let state = agent.state.clone();

        // Update the agent state
        agent.update(
            &state,
            &action,
            reward,
            &next_state,
            &self.actions, // THIS SHOULD POSSIBLY VARY
        );

        agent.update_state(next_state);

        (new_position, done)
    }
}
