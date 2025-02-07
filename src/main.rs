use std::{collections::HashMap, rc::Rc};

use agent::{
    learning_agent::{Done, LearningAgent, Reward, StepFunction},
    state::{to_value, State, Value},
};
use environment::environment::Env;
use interface::{
    context::Context,
    grid::GridSize,
    keymapping::apply_input,
    settings::Settings,
    ui::{default_skin, keymappings_skin, show_debug_info, show_keymapping, show_settings},
};
use macroquad::{prelude::*, ui::root_ui};
use scheduler::scheduler::{Position, Scheduler};

pub mod agent;
pub mod environment;
pub mod interface;
pub mod scheduler;

#[macroquad::main("MASim")]
async fn main() {
    let mut settings = Settings::builder()
        .skin(HashMap::from([
            ("Default".to_string(), default_skin().await),
            ("Keymapping".to_string(), keymappings_skin().await),
        ]))
        .build()
        .await;

    let mut camera =
        Camera2D::from_display_rect(Rect::new(0., 0., screen_width(), screen_height()));

    let mut ctx = Context {
        grid_size: vec2(8., 8.),
        camera,
    };

    let env = Env::new(
        vec2(screen_width() * 0.1, screen_height() * 0.1),
        GridSize {
            width: 16,
            heigth: 16,
        },
        Some(32.),
        vec!["up", "down", "left", "right"],
    );

    let mut scheduler = Scheduler::new(env);

    let runner_func: StepFunction = Rc::new(
        move |agent: &LearningAgent,
              env: &Env,
              position: Position,
              state: &State,
              action: &'static str|
              -> (Position, State, Reward, Done) {
            /***** DEFINE THE STATE HERE *************/
            /* will crash if incorrect types defined */
            // println!("state: {:?}", state); // DEBUG
            let mut x: i32 = state[0].eq_type();
            let mut y: i32 = state[1].eq_type();
            /*****************************************/

            let (mut new_x, mut new_y) = (position.x, position.y);

            match action {
                "up" => new_y -= 1,
                "down" => new_y += 1,
                "left" => new_x -= 1,
                "right" => new_x += 1,
                _ => {}
            }

            let new_position = Position { x: new_x, y: new_y };
            x = new_x;
            y = new_y;

            if !env.valid_position(new_position) {
                return (position, state.clone(), -5., false);
            }

            let mut reward: Reward = -1.;

            let prev_distance = (((env.goal.0 - position.x).pow(2)
                + (env.goal.1 - position.y).pow(2)) as f32)
                .sqrt();
            let current_distance =
                (((env.goal.0 - new_x).pow(2) + (env.goal.1 - new_y).pow(2)) as f32).sqrt();

            if current_distance < prev_distance {
                reward += 10.;
            }

            if env.prefered_cells.contains(&(new_x, new_y)) {
                reward += 10.;
            }

            if (new_x, new_y) == env.goal {
                reward += 50.;
                return (new_position, vec![to_value(x), to_value(y)], reward, true);
            }

            (new_position, vec![to_value(x), to_value(y)], reward, false)
        },
    );

    let n = 10;
    scheduler.add_agents(
        n,
        Some(IVec2 { x: 0, y: 0 }),
        YELLOW,
        "runner",
        vec![
            Value::VI32(0), // x
            Value::VI32(0), // y
        ],
        None,
        None,
        None,
        &runner_func,
    );

    scheduler.train_agents(10000);
    for i in 0..n {
        let (position, _, _) = &mut scheduler.agents[i];
        // let agent = agent.borrow_mut();
        position.x = 0;
        position.y = 0;
        // position = &mut IVec2 { x: 0, y: 0 };
    }

    // let wolf_func: StepFunction = Rc::new(
    //     move |agent: &LearningAgent,
    //           env: &Env,
    //           position: Position,
    //           state: &State,
    //           action: &'static str|
    //           -> (Position, State, Reward, Done) {
    //         /***** DEFINE THE STATE HERE *************/
    //         /* will crash if incorrect types defined */
    //         // println!("state: {:?}", state); // DEBUG
    //         let mut energy: u32 = state[0].eq_type();
    //         let mut p_reproduce: f32 = state[1].eq_type();
    //         let mut energy_from_food: u32 = state[2].eq_type();
    //         /*****************************************/
    //         let (mut new_x, mut new_y) = (position.x, position.y);

    //         match action {
    //             "up" => new_y -= 1,
    //             "down" => new_y += 1,
    //             "left" => new_x -= 1,
    //             "right" => new_x += 1,
    //             _ => {}
    //         }

    //         let new_position = Position { x: new_x, y: new_y };

    //         if !env.valid_position(new_position) {
    //             return (position, state.clone(), -5., false);
    //         }

    //         let mut reward: Reward = -1.;

    //         let prev_distance = (((env.goal.0 - position.x).pow(2)
    //             + (env.goal.1 - position.y).pow(2)) as f32)
    //             .sqrt();
    //         let current_distance =
    //             (((env.goal.0 - new_x).pow(2) + (env.goal.1 - new_y).pow(2)) as f32).sqrt();

    //         if current_distance < prev_distance {
    //             reward = 10.;
    //         }

    //         if env.prefered_cells.contains(&(new_x, new_y)) {
    //             reward += 10.;
    //         }

    //         if (new_x, new_y) == env.goal {
    //             reward += 50.;
    //             return (
    //                 new_position,
    //                 vec![
    //                     to_value(energy),
    //                     to_value(p_reproduce),
    //                     to_value(energy_from_food),
    //                 ],
    //                 reward,
    //                 true,
    //             );
    //         }

    //         (
    //             new_position,
    //             vec![
    //                 to_value(energy),
    //                 to_value(p_reproduce),
    //                 to_value(energy_from_food),
    //             ],
    //             reward,
    //             false,
    //         )
    //     },
    // );

    // scheduler.add_agent(
    //     Some(IVec2 { x: 0, y: 0 }),
    //     Color::from_rgba(127, 110, 119, 255),
    //     "wolf",
    //     vec![
    //         Value::VU32(10),                         // energy
    //         Value::VFloat(0.2_f32.to_bits() as u32), // p_reproduce (asexual)
    //         Value::VU32(10),                         // energy_from_food
    //     ],
    //     None,
    //     None,
    //     None,
    //     &wolf_func,
    // );

    // scheduler.add_agent(
    //     Some(IVec2 { x: 15, y: 15 }),
    //     Color::from_rgba(127, 110, 119, 255),
    //     "wolf",
    //     vec![
    //         Value::VU32(10),                         // energy
    //         Value::VFloat(0.2_f32.to_bits() as u32), // p_reproduce
    //         Value::VU32(10),                         // energy_from_food
    //     ],
    //     None,
    //     None,
    //     None,
    //     &wolf_func,
    // );

    // scheduler.add_agent(
    //     Some(IVec2 { x: 0, y: 0 }),
    //     WHITE,
    //     "sheep",
    //     vec![Value::VU32(5)],
    //     None,
    //     None,
    //     None,
    //     &wolf_func, // TODO change
    // );

    loop {
        settings.display_background();

        // User input
        if is_key_pressed(KeyCode::Q) {
            break;
        }
        apply_input(&mut ctx, &mut settings);

        draw_rectangle(
            (screen_width() * 0.1) + (8. * 32.),
            (screen_height() * 0.1) + (8. * 32.),
            32.,
            32.,
            GREEN,
        );

        scheduler.take_step();

        // 2D context
        set_default_camera();

        scheduler.display_env(
            vec2(screen_width() * 0.1, screen_height() * 0.1),
            32.,
            settings.text_color,
        );

        // Buttons
        let (_, skin) = settings.skin.get_key_value(&"Default".to_string()).unwrap();
        root_ui().push_skin(skin);
        if root_ui().button(vec2(screen_width() - 80., 20.), "Settings  ") {
            settings.toggle_display_settings();
        }

        if root_ui().button(vec2(screen_width() - 80., 40.), "Keymapping") {
            settings.toggle_display_keymapping();
        }

        root_ui().pop_skin();

        #[cfg_attr(any(), rustfmt::skip)]
        { // Display settings related informations
        if settings.display_settings   { show_settings(&mut settings); }
        if settings.display_keymapping { show_keymapping(&mut settings); }
        if settings.debug { show_debug_info(&ctx, &settings); }
        }

        next_frame().await
    }
}
