use std::{collections::HashMap, hash::Hash};

use macroquad::{math::UVec2, rand::ChooseRandom};

pub enum AgentType {}

#[derive(PartialEq, Eq, Hash)]
pub struct Q {
    state: UVec2,
    action: &'static str,
}

pub struct LearningAgent {
    /// unique if of the agent
    id: u32,
    /// position in a 2D graphe
    position: UVec2,
    /// reward for a given state and action
    // q_values: HashMap<u32, f32>,
    q_values: HashMap<Q, f32>,
    /// learning rate
    alpha: f32,
    /// discount factor
    gamma: f32,
    /// exploration rate
    epsilon: f32,
}

impl LearningAgent {
    pub fn new(
        id: u32,
        position: UVec2,
        alpha: Option<f32>,
        gamma: Option<f32>,
        epsilon: Option<f32>,
    ) -> LearningAgent {
        let alpha = alpha.unwrap_or(0.1);
        let gamma = gamma.unwrap_or(0.9);
        let epsilon = epsilon.unwrap_or(0.2);

        LearningAgent {
            id,
            position,
            q_values: HashMap::new(),
            alpha,
            gamma,
            epsilon,
        }
    }

    pub fn get_q_value(&self, state: UVec2, action: &'static str) -> &f32 {
        let k = Q { state, action };

        match self.q_values.get(&k) {
            Some(value) => value,
            None => panic!("get_q_value(...): no value found"),
        }
    }

    pub fn set_q_value(&mut self, state: UVec2, action: &'static str, value: f32) {
        self.q_values.insert(Q { state, action }, value);
    }

    pub fn move_to(&mut self, position: UVec2) {
        self.position = position;
    }

    /// Epsilon-greedy action selection
    pub fn choose_action(&self, state: UVec2, actions: &Vec<&'static str>) -> &'static str {
        if rand::random_range(0.0..1.) < self.epsilon {
            return actions.choose().unwrap();
        }

        let q_values: HashMap<&Q, &f32> = self
            .q_values
            .iter()
            .filter(|(k, _)| k.state == state && actions.contains(&k.action))
            .collect();

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn choosing_action() {
        let mut agent = LearningAgent::new(0, UVec2 { x: 0, y: 0 }, None, None, Some(0.));
        let actions = vec!["eat", "move", "dance", "sing"];

        assert_eq!(agent.choose_action(UVec2 { x: 0, y: 0 }, &actions), "");

        agent.set_q_value(UVec2 { x: 0, y: 0 }, "eat", 0.);
        agent.set_q_value(UVec2 { x: 0, y: 0 }, "move", 1.);
        agent.set_q_value(UVec2 { x: 0, y: 0 }, "dance", 2.);
        agent.set_q_value(UVec2 { x: 0, y: 0 }, "sing", 3.);

        assert_eq!(agent.choose_action(UVec2 { x: 0, y: 0 }, &actions), "sing");
        assert_ne!(agent.choose_action(UVec2 { x: 1, y: 0 }, &actions), "sing");

        agent.set_q_value(UVec2 { x: 0, y: 0 }, "eat", 4.);

        assert_eq!(agent.choose_action(UVec2 { x: 0, y: 0 }, &actions), "eat");
        assert_ne!(agent.choose_action(UVec2 { x: 1, y: 0 }, &actions), "eat");

        agent.set_q_value(UVec2 { x: 0, y: 0 }, "move", 4.);

        let mut count_eat = 0;
        let mut count_move = 0;

        for _ in 0..1000 {
            let result = agent.choose_action(UVec2 { x: 0, y: 0 }, &actions);

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
