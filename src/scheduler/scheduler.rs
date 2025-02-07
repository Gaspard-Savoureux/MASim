use std::{cell::RefCell, collections::HashMap, rc::Rc};

use macroquad::{
    color::Color,
    math::{IVec2, Vec2},
};

use crate::{
    agent::{
        learning_agent::{LearningAgent, StepFunction},
        state::State,
    },
    environment::environment::Env,
};

pub type AgentRef = Rc<RefCell<LearningAgent>>;
pub type Position = IVec2;
pub struct Scheduler {
    pub agents: Vec<(Position, Color, AgentRef)>,
    pub agents_per_types: HashMap<&'static str, Vec<AgentRef>>,
    pub env: Env,
    /// This is the count of id and next id to be given to an agent
    current_id: u32,
    // pub function_step: HashMap<&'static str, StepFunction>,
}

impl Scheduler {
    pub fn new(env: Env) -> Self {
        Scheduler {
            agents: Vec::new(),
            agents_per_types: HashMap::new(),
            env,
            current_id: 0,
        }
    }

    pub fn display_env(&mut self, origin: Vec2, cell_size: f32, grid_color: Color) {
        let agent_positions: Vec<(Position, Color)> = self
            .agents
            .iter()
            .map(|(position, color, _)| (*position, *color))
            .collect();

        self.env
            .display_grid(origin, cell_size, grid_color, agent_positions);
    }

    fn generate_id(&mut self) -> u32 {
        self.current_id += 1;
        self.current_id
    }

    /// Add **ONE** agent
    pub fn add_agent(
        &mut self,
        position: Option<Position>,
        color: Color,
        agent_type: &'static str,
        state: State,
        learning_rate: Option<f32>,
        discount_factor: Option<f32>,
        exploration_rate: Option<f32>,
        step_fn: &StepFunction,
    ) {
        let position = position.unwrap_or(Position {
            x: rand::random_range(0..*self.env.get_width() as i32),
            y: rand::random_range(0..*self.env.get_heigth() as i32),
        });

        let new_agent = Rc::new(RefCell::new(LearningAgent::new(
            self.generate_id(),
            agent_type,
            state,
            learning_rate,
            discount_factor,
            exploration_rate,
            step_fn,
        )));

        // Add new agent in Vector with all the other agents
        self.agents.push((position, color, new_agent.clone()));

        // Add new agent in agents_per_types
        let agents = self.agents_per_types.get_mut(agent_type);
        match agents {
            Some(list) => list.push(new_agent),
            None => {
                let _ = self.agents_per_types.insert(agent_type, vec![new_agent]);
            }
        }
    }

    /// Add **Multiple** agents
    pub fn add_agents(
        &mut self,
        n: usize,
        position: Option<Position>,
        color: Color,
        agent_type: &'static str,
        state: State,
        learning_rate: Option<f32>,
        discount_factor: Option<f32>,
        exploration_rate: Option<f32>,
        step_fn: &StepFunction,
    ) {
        let mut new_agents: Vec<(Position, Color, AgentRef)> = Vec::with_capacity(n);

        for _ in 0..n {
            let position = position.unwrap_or(Position {
                x: rand::random_range(0..*self.env.get_width() as i32),
                y: rand::random_range(0..*self.env.get_heigth() as i32),
            });

            let new_agent = Rc::new(RefCell::new(LearningAgent::new(
                self.generate_id(),
                agent_type,
                state.clone(),
                learning_rate,
                discount_factor,
                exploration_rate,
                step_fn,
            )));

            new_agents.push((position, color, new_agent));
        }

        // Add all the new agents in Vector with all the other agents
        self.agents.append(&mut new_agents);

        // Add new agent in agents_per_types
        let agents = self.agents_per_types.get_mut(agent_type);

        let mut new_agents_type: Vec<AgentRef> = new_agents
            .iter()
            .map(|(_, _, agent)| agent.clone())
            .collect();
        match agents {
            Some(list) => {
                list.append(&mut new_agents_type);
            }
            None => {
                let _ = self.agents_per_types.insert(agent_type, new_agents_type);
            }
        }
    }

    pub fn take_step(&mut self) {
        // Iterate over agents and remove those that are done
        for i in (0..self.agents.len()).rev() {
            let mut remove = false;
            {
                let (position, _, agent) = &mut self.agents[i];

                let (new_position, done) = self.env.step(*position, agent);

                // update new position
                *position = new_position;

                let agent = agent.borrow();
                if done {
                    println!("DONE");

                    remove = true;

                    match self.agents_per_types.get_mut(agent.agent_type) {
                      // NOTE: Could be replaced by hashmap for faster delete
                      Some(agents) => agents.retain(|a| a.borrow().id != agent.id),
                      None => panic!("Trying to remove agent from inexisting type. This is not supposed to be possible :|"),
                    }
                }
            }

            // Remove agent if done
            if remove {
                self.agents.remove(i);
            }
        }

        // DEBUG
        // println!("nb agents in agents: {}", self.agents.len());
        // println!("nb agents per types:");
        // for (agent_type, agents) in self.agents_per_types.clone() {
        //     println!("\t agent_type: {}, nb: {}", agent_type, agents.len());
        // }
    }

    pub fn train_agents(&mut self, nb_steps: u32) {
        for step in 0..nb_steps {
            for i in (0..self.agents.len()).rev() {
                let (position, _, agent) = &mut self.agents[i];

                let (new_position, _) = self.env.step(*position, agent);

                // update new position
                *position = new_position;
            }

            // Print progression
            println!(
                "Training agents progressions: {}%",
                (step as f32 / nb_steps as f32) * 100.
            );

            // DEBUG
            // println!("nb agents in agents: {}", self.agents.len());
            // println!("nb agents per types:");
            // for (agent_type, agents) in self.agents_per_types.clone() {
            //     println!("\t agent_type: {}, nb: {}", agent_type, agents.len());
            // }
        }

        // DEBUG
        // println!("nb agents in agents: {}", self.agents.len());
        // println!("nb agents per types:");
        // for (agent_type, agents) in self.agents_per_types.clone() {
        //     println!("\t agent_type: {}, nb: {}", agent_type, agents.len());
        // }
    }
}
