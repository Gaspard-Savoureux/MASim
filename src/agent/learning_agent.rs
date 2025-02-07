use std::{collections::HashMap, fmt::Debug, hash::Hash, rc::Rc};

use macroquad::rand::ChooseRandom;

use crate::{environment::environment::Env, scheduler::scheduler::Position};

use super::state::State;

pub type Done = bool;
pub type StepFunction = Rc<
    dyn Fn(&LearningAgent, &Env, Position, &State, &'static str) -> (Position, State, Reward, Done),
>;
pub type Reward = f32;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Q {
    state: State,
    action: &'static str,
}

#[derive()]
pub struct LearningAgent {
    /// unique id of the agent
    pub id: u32,
    /// The type of the agent. Example: wolf, sheep, etc.
    pub agent_type: &'static str,
    /// position in a 2D graphe
    pub state: State,
    /// Q-values
    q_values: HashMap<Q, f32>,
    /// alpha / learning rate
    learning_rate: f32,
    /// gamma / discount factor
    discount_factor: f32,
    /// epsilon / exploration rate
    exploration_rate: f32,
    /// Function representing a step
    step_fn: StepFunction,
}

impl LearningAgent {
    pub fn new(
        id: u32,
        agent_type: &'static str,
        state: State,
        learning_rate: Option<f32>,
        discount_factor: Option<f32>,
        exploration_rate: Option<f32>,
        step_fn: &StepFunction,
    ) -> LearningAgent {
        let learning_rate = learning_rate.unwrap_or(0.1);
        let discount_factor = discount_factor.unwrap_or(0.9);
        let exploration_rate = exploration_rate.unwrap_or(0.2);

        LearningAgent {
            id,
            agent_type,
            state,
            q_values: HashMap::new(),
            learning_rate,
            discount_factor,
            exploration_rate,
            step_fn: Rc::clone(step_fn),
        }
    }

    pub fn get_q_value(&self, state: State, action: &'static str) -> &f32 {
        let k = Q { state, action };

        match self.q_values.get(&k) {
            Some(value) => value,
            None => &0.,
        }
    }

    pub fn set_q_value(&mut self, state: State, action: &'static str, value: f32) {
        self.q_values.insert(Q { state, action }, value);
    }

    pub fn update_state(&mut self, state: State) {
        self.state = state;
    }

    /// Returns subset of q values with the same state and actions
    fn q_values_subset(&self, state: &State, actions: &Vec<&'static str>) -> HashMap<&Q, &f32> {
        self.q_values
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

    /// Epsilon-greedy action selection
    pub fn choose_action(&self, state: &State, actions: &Vec<&'static str>) -> &'static str {
        if rand::random_range(0.0..1.) < self.exploration_rate || self.q_values.is_empty() {
            return actions.choose().unwrap();
        }

        let q_values = self.q_values_subset(state, actions);

        let max_entry = q_values.iter().max_by(|a, b| a.1.total_cmp(b.1));

        if let Some((_, val)) = max_entry {
            let max = val;
            let possible_actions: HashMap<_, _> =
                q_values.iter().filter(|(_, &v)| v == *max).collect();
            let possible_actions: Vec<_> =
                possible_actions.iter().map(|(&q, _)| q.action).collect();
            return possible_actions.choose().unwrap();
        }

        ""
    }

    /// Q-learning update
    ///
    /// Q-learning update rule:
    /// Q(s, a) <- Q(s, a) + alpha * (reward + gamma * max_a' Q(s', a') - Q(s, a))
    pub fn update(
        &mut self,
        state: &State,
        action: &'static str,
        reward: f32,
        next_state: &State,
        next_actions: &Vec<&'static str>,
    ) {
        let old_q_value = self.get_q_value(state.clone(), action);
        let future_q_value = self.max_q_val(&self.q_values_subset(next_state, next_actions));

        // Q-learning update
        let new_q_value = old_q_value
            + self.learning_rate * (reward + self.discount_factor * future_q_value - old_q_value);
        self.set_q_value(state.clone(), action, new_q_value);
    }

    pub fn step(
        &self,
        env: &Env,
        position: Position,
        state: &State,
        action: &'static str,
    ) -> (Position, State, Reward, Done) {
        (self.step_fn)(&self, env, position, state, action)
    }
}

#[cfg(test)]
mod tests {
    use crate::agent::state::Value;

    use super::*;

    #[test]
    fn choosing_action() {
        let agent_type = "wolf";
        let func: StepFunction = Rc::new(
            move |_agent: &LearningAgent,
                  _env: &Env,
                  _position: Position,
                  _state: &State,
                  _action: &'static str|
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
        );

        let actions = vec!["eat", "move", "dance", "sing"];

        // assert_eq!(agent.choose_action(default_state, &actions), "");

        agent.set_q_value(default_state.clone(), "eat", 0.);
        agent.set_q_value(default_state.clone(), "move", 1.);
        agent.set_q_value(default_state.clone(), "dance", 2.);
        agent.set_q_value(default_state.clone(), "sing", 3.);

        assert_eq!(agent.choose_action(&default_state, &actions), "sing");
        assert_ne!(
            agent.choose_action(&vec![Value::VBool(true)], &actions),
            "sing"
        );

        agent.set_q_value(vec![Value::VBool(true)], "eat", 4.);

        assert_eq!(agent.choose_action(&default_state, &actions), "eat");
        assert_ne!(
            agent.choose_action(&vec![Value::VBool(false)], &actions),
            "eat"
        );

        agent.set_q_value(vec![Value::VBool(false)], "move", 4.);

        let mut count_eat = 0;
        let mut count_move = 0;

        for _ in 0..1000 {
            let result = agent.choose_action(&default_state, &actions);

            match result {
                "eat" => count_eat += 1,
                "move" => count_move += 1,
                _ => panic!("The result must be either eat or move"),
            }
            assert!(result.eq("eat") || result.eq("move"));
        }

        // Can technically be false but statistically unprobable
        assert!(count_eat > 0);
        assert!(count_move > 0);
    }
}
