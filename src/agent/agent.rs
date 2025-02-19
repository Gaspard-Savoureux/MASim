use std::{collections::HashMap, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::{environment::environment::Env, scheduler::scheduler::Position};

use super::{learning_agent::LearningAgent, state::State, swarm_agent::SwarmAgent};

pub type QTable = HashMap<Q, f32>;
pub type Reward = f32;
pub type Done = bool;
pub type Action = u32;

pub type StepFunction<A> =
    Rc<dyn Fn(&A, &mut Env, Position, &State, &Action) -> (Position, State, Reward, Done)>;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Q {
    pub state: State,
    pub action: u32,
}

pub enum Agent {
    Learning(LearningAgent),
    Swarm(SwarmAgent),
}

pub trait IsAgent {
    fn get_unique_id(&self) -> u32;

    fn get_type(&self) -> &'static str;

    fn get_state(&self) -> &State;

    fn set_state(&mut self, state: State);

    fn get_q_value(&self, state: State, action: u32) -> f32;

    fn set_q_value(&mut self, state: State, action: u32, value: f32);

    /// Saves the q_table to a file
    fn save_q_table(&self, filepath: &str);

    /// Load a q_table from a file
    fn load_q_table(&mut self, filepath: &str);

    /// Epsilon-greedy action selection
    fn choose_action(&self, state: &State, actions: &Vec<u32>) -> u32;

    /// Q-learning update
    fn update(
        &mut self,
        state: &State,
        action: &u32,
        reward: f32,
        next_state: &State,
        next_actions: &Vec<u32>,
    );

    fn step(
        &self,
        env: &mut Env,
        position: Position,
        state: &State,
        action: &Action,
    ) -> (Position, State, Reward, Done);
}

impl IsAgent for Agent {
    fn get_unique_id(&self) -> u32 {
        match self {
            Agent::Learning(learning_agent) => learning_agent.get_unique_id(),
            Agent::Swarm(swarm_agent) => swarm_agent.get_unique_id(),
        }
    }

    fn get_type(&self) -> &'static str {
        match self {
            Agent::Learning(learning_agent) => learning_agent.get_type(),
            Agent::Swarm(swarm_agent) => swarm_agent.get_type(),
        }
    }

    fn get_state(&self) -> &State {
        match self {
            Agent::Learning(learning_agent) => learning_agent.get_state(),
            Agent::Swarm(swarm_agent) => swarm_agent.get_state(),
        }
    }

    fn set_state(&mut self, state: State) {
        match self {
            Agent::Learning(learning_agent) => learning_agent.set_state(state),
            Agent::Swarm(swarm_agent) => swarm_agent.set_state(state),
        }
    }

    fn get_q_value(&self, state: State, action: u32) -> f32 {
        match self {
            Agent::Learning(learning_agent) => learning_agent.get_q_value(state, action),
            Agent::Swarm(swarm_agent) => swarm_agent.get_q_value(state, action),
        }
    }

    fn set_q_value(&mut self, state: State, action: u32, value: f32) {
        match self {
            Agent::Learning(learning_agent) => learning_agent.set_q_value(state, action, value),
            Agent::Swarm(swarm_agent) => swarm_agent.set_q_value(state, action, value),
        };
    }

    fn save_q_table(&self, filepath: &str) {
        match self {
            Agent::Learning(learning_agent) => learning_agent.save_q_table(filepath),
            Agent::Swarm(swarm_agent) => swarm_agent.save_q_table(filepath),
        }
    }

    fn load_q_table(&mut self, filepath: &str) {
        match self {
            Agent::Learning(learning_agent) => learning_agent.load_q_table(filepath),
            Agent::Swarm(swarm_agent) => swarm_agent.load_q_table(filepath),
        }
    }

    fn choose_action(&self, state: &State, actions: &Vec<u32>) -> u32 {
        match self {
            Agent::Learning(learning_agent) => learning_agent.choose_action(state, actions),
            Agent::Swarm(swarm_agent) => swarm_agent.choose_action(state, actions),
        }
    }

    fn update(
        &mut self,
        state: &State,
        action: &u32,
        reward: f32,
        next_state: &State,
        next_actions: &Vec<u32>,
    ) {
        match self {
            Agent::Learning(learning_agent) => {
                learning_agent.update(state, action, reward, next_state, next_actions)
            }
            Agent::Swarm(swarm_agent) => {
                swarm_agent.update(state, action, reward, next_state, next_actions)
            }
        }
    }

    fn step(
        &self,
        env: &mut Env,
        position: Position,
        state: &State,
        action: &Action,
    ) -> (Position, State, Reward, Done) {
        match self {
            Agent::Learning(learning_agent) => learning_agent.step(env, position, state, action),
            Agent::Swarm(swarm_agent) => swarm_agent.step(env, position, state, action),
        }
    }
}
