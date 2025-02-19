use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, BufWriter},
    rc::Rc,
};

use crate::{environment::environment::Env, scheduler::scheduler::Position};

use super::{
    agent::{Action, Done, IsAgent, QTable, Reward, StepFunction},
    state::State,
};

#[derive()]
pub struct SwarmAgent {
    /// unique id of the agent
    pub id: u32,
    /// The type of the agent. Example: wolf, sheep, etc.
    pub agent_type: &'static str,
    /// position in a 2D graphe
    pub state: State,
    /// Q-values
    q_table: QTable,
    // q_values: HashMap<Q, f32>,
    /// alpha / learning rate
    pub learning_rate: f32,
    /// gamma / discount factor
    pub discount_factor: f32,
    /// epsilon / exploration rate
    pub exploration_rate: f32,
    /// Function representing a step
    step_fn: StepFunction<SwarmAgent>,
}

impl IsAgent for SwarmAgent {
    fn get_unique_id(&self) -> u32 {
        self.id
    }

    fn get_type(&self) -> &'static str {
        self.agent_type
    }

    fn get_state(&self) -> &State {
        &self.state
    }

    fn set_state(&mut self, state: State) {
        self.state = state;
    }

    fn get_q_value(&self, state: State, action: u32) -> &f32 {
        todo!()
    }

    fn set_q_value(&mut self, state: State, action: u32, value: f32) {
        todo!()
    }

    fn save_q_table(&self, filepath: &str) {
        let file = File::create(filepath).expect("Failed to create file");
        let mut writer = BufWriter::new(file);

        bincode::serialize_into(&mut writer, &self.q_table).expect("Failed to write q_table");
    }

    fn load_q_table(&mut self, filepath: &str) {
        let file = match File::open(filepath) {
            Ok(file) => file,
            Err(_) => return, // We do not wish to crash if the file is non-existant
        };

        let mut reader = BufReader::new(file);

        let q_table: QTable =
            bincode::deserialize_from(&mut reader).expect("Failed to read q_table");
        self.q_table = q_table;
    }

    fn choose_action(&self, state: &State, action: &Vec<u32>) -> u32 {
        todo!()
    }

    fn update(
        &mut self,
        state: &State,
        action: &u32,
        reward: f32,
        next_state: &State,
        next_actions: &Vec<u32>,
    ) {
        todo!()
    }

    fn step(
        &self,
        env: &mut Env,
        position: Position,
        state: &State,
        action: &super::agent::Action,
    ) -> (Position, State, super::agent::Reward, super::agent::Done) {
        todo!()
    }
}

impl SwarmAgent {
    pub fn new(
        id: u32,
        agent_type: &'static str,
        state: State,
        learning_rate: Option<f32>,
        discount_factor: Option<f32>,
        exploration_rate: Option<f32>,
        step_fn: &StepFunction<SwarmAgent>,
        q_table_filepath: Option<&str>,
    ) -> Self {
        let learning_rate = learning_rate.unwrap_or(0.1);
        let discount_factor = discount_factor.unwrap_or(0.9);
        let exploration_rate = exploration_rate.unwrap_or(0.2);

        let mut new_agent = SwarmAgent {
            id,
            agent_type,
            state,
            q_table: HashMap::new(),
            learning_rate,
            discount_factor,
            exploration_rate,
            step_fn: Rc::clone(step_fn),
        };

        if let Some(filepath) = q_table_filepath {
            new_agent.load_q_table(filepath);
        }

        new_agent
    }
}
