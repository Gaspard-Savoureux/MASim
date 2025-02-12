use std::{cell::RefCell, collections::HashMap, rc::Rc};

use macroquad::{
    color::{GREEN, YELLOW},
    math::{vec2, IVec2},
    window::{screen_height, screen_width},
};
use masim::define_const;

use crate::{
    agent::{
        learning_agent::{Action, Done, LearningAgent, Reward, StepFunction},
        state::{to_value, State, Value},
    },
    environment::environment::Env,
    interface::grid::GridSize,
    scheduler::scheduler::{Position, Scheduler},
};

define_const!(ACTIONS => UP, DOWN, LEFT, RIGHT);
define_const!(ENV_DATA => GOAL);

pub fn main() -> Scheduler {
    // The file that will save the trained data set
    let q_table_filepath = "trained_runner.bin";

    let goal = IVec2 { x: 8, y: 8 };
    let persistent_elements = HashMap::from([(goal, GREEN)]);
    let env = Env::new(
        vec2(screen_width() * 0.1, screen_height() * 0.1),
        vec2(screen_width() * 0.9, screen_height() * 0.9),
        GridSize {
            width: 16,
            heigth: 16,
        },
        persistent_elements,
        ACTIONS,
        HashMap::from([(
            GOAL,
            to_value((goal.x, goal.y)), // (x, y)
        )]),
    );

    let mut scheduler = Scheduler::new(env);

    let runner_func: StepFunction = Rc::new(
        move |_agent: &LearningAgent,
              env: &mut Env,
              position: Position,
              state: &State,
              action: &Action|
              -> (Position, State, Reward, Done) {
            /***** DEFINE THE STATE HERE *************/
            /* will crash if incorrect types defined */
            // println!("state: {:?}", state); // DEBUG
            let above: bool = state[0].eq_type();
            let below: bool = state[1].eq_type();
            let left: bool = state[2].eq_type();
            let right: bool = state[3].eq_type();
            /*****************************************/
            let (mut new_x, mut new_y) = (position.x, position.y);

            match action {
                &UP => new_y -= 1,
                &DOWN => new_y += 1,
                &LEFT => new_x -= 1,
                &RIGHT => new_x += 1,
                _ => {}
            }

            let new_position = Position { x: new_x, y: new_y };

            /************ UPDATING STATE *************/
            let (goal_x, goal_y): (i32, i32) = env.data.get(&GOAL).unwrap().eq_type();

            fn get_new_state(
                (new_x, new_y): (i32, i32),
                (goal_x, goal_y): (i32, i32),
            ) -> Vec<Value> {
                let above = new_y < goal_y;
                let below = new_y > goal_y;
                let left = new_x < goal_x;
                let right = new_x > goal_x;

                vec![
                    to_value(above),
                    to_value(below),
                    to_value(left),
                    to_value(right),
                ]
            }
            /*****************************************/

            /************ REWARD SYSTEM **************/
            let mut reward: Reward = -1.;

            // State doesn't check wall or border so we shall not change the reward
            if !env.valid_position(new_position) {
                let state = if !above && !below && !left && !right {
                    get_new_state((new_x, new_y), (goal_x, goal_y))
                } else {
                    vec![
                        to_value(above),
                        to_value(below),
                        to_value(left),
                        to_value(right),
                    ]
                };
                return (position, state, 0., false);
                // return (position, state.clone(), reward - 5., false);
            }

            let prev_distance =
                (((goal_x - position.x).pow(2) + (goal_y - position.y).pow(2)) as f32).sqrt();
            let current_distance =
                (((goal_x - new_x).pow(2) + (goal_y - new_y).pow(2)) as f32).sqrt();

            // When distance from goal is shorter
            if current_distance < prev_distance {
                reward += 10.;
            }

            // Goal reached
            if new_x == goal_x && new_y == goal_y {
                reward += 50.;

                // Generate new goal
                let new_goal = env.get_random_position();

                // Update goal
                env.move_persistent_element(
                    IVec2 {
                        x: goal_x,
                        y: goal_y,
                    },
                    new_goal,
                );

                env.data.insert(GOAL, to_value((new_goal.x, new_goal.y)));

                // Done set to false in order to keep the demo running
                return (
                    new_position,
                    get_new_state((new_x, new_y), (goal_x, goal_y)),
                    reward,
                    false,
                );
            }

            (
                new_position,
                get_new_state((new_x, new_y), (goal_x, goal_y)),
                reward,
                false,
            )
            /*****************************************/
        },
    );

    // Comment the following to remove training
    train_agent(&mut scheduler, &runner_func, q_table_filepath);

    let n = 10;
    scheduler.add_agents(
        n,
        // Some(IVec2 { x: 0, y: 0 }), // Uncomment for the same starting point
        None,
        YELLOW,
        "runner",
        vec![
            Value::VBool(true), // ABOVE_TARGET
            Value::VBool(true), // BELOW_TARGET
            Value::VBool(true), // LEFT_OF_TARGET
            Value::VBool(true), // RIGHT_OF_TARGET
        ],
        None,
        None,
        Some(0.01),
        &runner_func,
        Some(q_table_filepath),
    );

    scheduler
}

fn train_agent(scheduler: &mut Scheduler, step_fn: &StepFunction, q_table_filepath: &str) {
    let mut new_agent = Rc::new(RefCell::new(LearningAgent::new(
        1000,
        "runner",
        vec![
            Value::VBool(true), // ABOVE_TARGET
            Value::VBool(true), // BELOW_TARGET
            Value::VBool(true), // LEFT_OF_TARGET
            Value::VBool(true), // RIGHT_OF_TARGET
        ],
        None,
        None,
        Some(0.4),
        step_fn,
        Some(&q_table_filepath),
    )));

    scheduler.save_q_table_to_file(&mut new_agent, 1000, &q_table_filepath, true);
}
