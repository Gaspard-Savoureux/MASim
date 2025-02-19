use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, BufWriter},
    rc::Rc,
};

use macroquad::rand::ChooseRandom;

use crate::{environment::environment::Env, scheduler::scheduler::Position};

use super::{
    agent::{IsAgent, QTable, StepFunction, Q},
    state::State,
};

#[derive()]
pub struct LearningAgent {
    /// unique id of the agent
    pub id: u32,
    /// The type of the agent. Example: wolf, sheep, etc.
    pub agent_type: &'static str, // TODO change to u32 and use define_const!()
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
    step_fn: StepFunction<LearningAgent>,
}

impl IsAgent for LearningAgent {
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
        let k = Q { state, action };

        match self.q_table.get(&k) {
            Some(value) => value,
            None => &0.,
        }
    }

    fn set_q_value(&mut self, state: State, action: u32, value: f32) {
        self.q_table.insert(Q { state, action }, value);
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

    /// Epsilon-greedy action selection
    fn choose_action(&self, state: &State, actions: &Vec<u32>) -> u32 {
        if rand::random_range(0.0..1.) < self.exploration_rate || self.q_table.is_empty() {
            return *actions.choose().unwrap();
        }

        let q_values = self.q_values_subset(state, actions);

        let max_entry = q_values.iter().max_by(|a, b| a.1.total_cmp(b.1));

        if let Some((_, val)) = max_entry {
            let max = val;
            let possible_actions: HashMap<_, _> =
                q_values.iter().filter(|(_, &v)| v == *max).collect();
            let possible_actions: Vec<_> = possible_actions
                .iter()
                .map(|(&q, _)| q.action.clone())
                .collect();
            return *possible_actions.choose().unwrap();
        }

        return *actions.choose().unwrap();
    }

    /// Q-learning update
    ///
    /// Q-learning update rule:
    /// Q(s, a) <- Q(s, a) + alpha * (reward + gamma * max_a' Q(s', a') - Q(s, a))
    fn update(
        &mut self,
        state: &State,
        action: &u32,
        reward: f32,
        next_state: &State,
        next_actions: &Vec<u32>,
    ) {
        let old_q_value = self.get_q_value(state.clone(), *action);
        let future_q_value = self.max_q_val(&self.q_values_subset(next_state, next_actions));

        // Q-learning update
        let new_q_value = old_q_value
            + self.learning_rate * (reward + self.discount_factor * future_q_value - old_q_value);
        self.set_q_value(state.clone(), *action, new_q_value);
    }

    fn step(
        &self,
        env: &mut Env,
        position: Position,
        state: &State,
        action: &super::agent::Action,
    ) -> (Position, State, super::agent::Reward, super::agent::Done) {
        (self.step_fn)(&self, env, position, state, action)
    }
}

impl LearningAgent {
    pub fn new(
        id: u32,
        agent_type: &'static str,
        state: State,
        learning_rate: Option<f32>,
        discount_factor: Option<f32>,
        exploration_rate: Option<f32>,
        step_fn: &StepFunction<LearningAgent>,
        q_table_filepath: Option<&str>,
    ) -> Self {
        let learning_rate = learning_rate.unwrap_or(0.1);
        let discount_factor = discount_factor.unwrap_or(0.9);
        let exploration_rate = exploration_rate.unwrap_or(0.2);

        let mut new_agent = LearningAgent {
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

    /// Returns subset of q values with the same state and actions
    fn q_values_subset(&self, state: &State, actions: &Vec<u32>) -> HashMap<&Q, &f32> {
        self.q_table
            .iter()
            .filter(|(k, _)| k.state == *state && actions.contains(&k.action))
            .collect()
    }

    fn max_q_val(&self, q_values: &HashMap<&Q, &f32>) -> f32 {
        let max_entry = q_values.iter().max_by(|&a, &b| a.1.total_cmp(b.1));

        match max_entry {
            Some((_, val)) => **val,
            None => 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use masim::define_const;

    use crate::agent::{
        agent::{Action, Done, Reward},
        state::Value,
    };

    use super::*;

    #[test]
    fn choosing_action() {
        let agent_type = "wolf";
        let func: StepFunction<LearningAgent> = Rc::new(
            move |_agent: &LearningAgent,
                  _env: &mut Env,
                  _position: Position,
                  _state: &State,
                  _action: &Action|
                  -> (Position, State, Reward, Done) {
                (Position { x: 0, y: 0 }, vec![Value::VI32(32)], 0., true)
            },
        );
        // example: [energy, day_lived, bald]
        let default_state = vec![Value::VI32(4), Value::VU32(23456), Value::VBool(false)];

        let mut agent = LearningAgent::new(
            0,
            agent_type,
            default_state.clone(),
            None,
            None,
            Some(0.),
            &func,
            None,
        );

        define_const!(ACTIONS => EAT, MOVE, DANCE, SING);
        let actions = Vec::from(ACTIONS);

        agent.set_q_value(default_state.clone(), EAT, 0.);
        agent.set_q_value(default_state.clone(), MOVE, 1.);
        agent.set_q_value(default_state.clone(), DANCE, 2.);
        agent.set_q_value(default_state.clone(), SING, 3.);

        assert_eq!(agent.choose_action(&default_state, &actions), SING);
        assert_ne!(
            agent.choose_action(&vec![Value::VBool(true)], &actions),
            SING
        );

        agent.set_q_value(default_state.clone(), EAT, 4.);

        assert_eq!(agent.choose_action(&default_state, &actions), EAT);
        assert_ne!(
            agent.choose_action(&vec![Value::VBool(false)], &actions),
            EAT
        );

        agent.set_q_value(default_state.clone(), MOVE, 4.);

        let mut count_eat = 0;
        let mut count_move = 0;

        for _ in 0..1000 {
            let result = agent.choose_action(&default_state, &actions);

            match result {
                EAT => count_eat += 1,
                MOVE => count_move += 1,
                _ => panic!("The result must be either eat or move"),
            }
            assert!(result.eq(&EAT) || result.eq(&MOVE));
        }

        // Can technically be false but statistically improbable
        assert!(count_eat > 0);
        assert!(count_move > 0);
    }
}
