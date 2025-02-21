use std::{cell::RefCell, collections::HashMap, rc::Rc};

use macroquad::{
    color::{Color, BLACK, BLUE, ORANGE, PURPLE, RED, YELLOW},
    math::{vec2, IVec2},
    window::{screen_height, screen_width},
};
use masim::define_const;

use crate::{
    agent::{
        agent::{Action, Done, Reward, StepFunction},
        state::{to_value, State, Value},
        swarm_agent::{load_q_table, SwarmAgent},
    },
    environment::environment::Env,
    interface::grid::GridSize,
    scheduler::scheduler::{Position, Scheduler},
};

// Define your actions here
define_const!(ACTIONS => UP, DOWN, LEFT, RIGHT);
// Define your environment data here
define_const!(ENV_DATA => VISITS);

// map
/// World => HashMap<Position, CELLTYPE>
// type World = HashMap<(i32, i32), u32>;
/// Visits => HashMap<Position, num_visits>
type Visits = HashMap<(i32, i32), u32>;
const WIDTH: usize = 160;
const HEIGTH: usize = 160;
define_const!(CELLTYPE =>
    WALL,
    ROBOT,
    DISCOVERED_EMPTY,
    DISCOVERED_MINERAL,
    JUST_DISCOVERED_EMPTY,
    JUST_DISCOVERED_MINERAL
);
// TODO change the colors (they are ugly as ****)
define_const!(COLOR_SCHEME: Color =>
    BASE_MINERAL: BLACK,
    JUST_DISCOVERED_EMPTY_COLOR: ORANGE,
    DISCOVERED_EMPTY_COLOR: YELLOW,
    JUST_DISCOVERED_MINERAL_COLOR: RED,
    DISCOVERED_MINERAL_COLOR: PURPLE
);

// bots
const FOV: u32 = 2;

pub fn main() -> Scheduler {
    // The file that will save the trained data set
    let q_table_filepath = "robot_explorer.bin";

    // Get random procedural map generation
    let blob_positions = generate_map(
        GridSize {
            width: WIDTH,
            heigth: HEIGTH,
        },
        0.1,
        10,
    );

    let mut persistent_elements = HashMap::new();
    for (x, y) in blob_positions.clone() {
        persistent_elements.insert(
            IVec2 {
                x: x as i32,
                y: y as i32,
            },
            BASE_MINERAL,
        );
    }

    let visits: Visits = HashMap::new();

    let env = Env::new(
        vec2(screen_width() * 0.1, screen_height() * 0.1),
        vec2(screen_width() * 0.9, screen_height() * 0.9),
        GridSize {
            width: WIDTH,
            heigth: HEIGTH,
        },
        persistent_elements.clone(),
        ACTIONS,
        // HashMap::new(),
        // HashMap::from([(WORLD, to_value(world)), (VEINS, to_value(blob_positions))]),
        HashMap::from([(VISITS, to_value(visits.clone()))]),
    );

    let mut scheduler = Scheduler::new(env);

    let agent_func: StepFunction<SwarmAgent> = Rc::new(
        move |_agent: &SwarmAgent,
              env: &mut Env,
              position: Position,
              state: &State,
              action: &Action|
              -> (Position, State, Reward, Done) {
            /***** DEFINE THE STATE HERE *************/
            /* will crash if incorrect types defined */
            // println!("state: {:?}", state); // DEBUG
            let surrounding_cells: Vec<u32> = state[0].eq_type();
            /*****************************************/
            let (mut new_x, mut new_y) = (position.x, position.y);

            // TODO add movements for omni-directionnal ones
            match action {
                &UP => new_y -= 1,
                &DOWN => new_y += 1,
                &LEFT => new_x -= 1,
                &RIGHT => new_x += 1,
                _ => {}
            }

            let new_position = Position { x: new_x, y: new_y };

            /************ UPDATING STATE *************/
            let new_cells = get_robot_state(position, env, FOV as i32);
            let mut new_grid = Vec::new();

            for (position, cell_type) in new_cells {
                let (x, y) = position;

                new_grid.push(cell_type);

                match cell_type {
                    DISCOVERED_EMPTY => {
                        env.update_persistent_element(IVec2 { x, y }, DISCOVERED_EMPTY_COLOR)
                    }
                    DISCOVERED_MINERAL => {
                        env.update_persistent_element(IVec2 { x, y }, DISCOVERED_MINERAL_COLOR)
                    }
                    JUST_DISCOVERED_EMPTY => {
                        env.update_persistent_element(IVec2 { x, y }, JUST_DISCOVERED_EMPTY_COLOR)
                    }
                    JUST_DISCOVERED_MINERAL => {
                        env.update_persistent_element(IVec2 { x, y }, JUST_DISCOVERED_MINERAL_COLOR)
                    }
                    ROBOT | WALL => {}
                    cell_type => println!("uncovered cell_type: {}", cell_type),
                }
            }
            // if surrounding_cells.is_empty() {
            //     surrounding_cells = get_robot_state(position, env, FOV as i32);
            // }
            /*****************************************/

            /************ REWARD SYSTEM **************/
            let mut reward: Reward = -1.;

            let inbound = env.position_inbound(new_position);
            let visits = env.data.get_mut(&VISITS).unwrap().as_map_mut().unwrap();

            // Next move out of bound
            if !inbound {
                if surrounding_cells.len() > 1 {
                    update_visits(visits, to_value((position.x, position.y)));
                    return (position, vec![to_value(surrounding_cells)], -40., false);
                } else {
                    update_visits(visits, to_value((new_x, new_y)));
                    return (position, vec![to_value(new_grid)], -40., false);
                }
            }

            let num_visits = update_visits(visits, to_value((new_x, new_y)));
            for cell in new_grid.clone() {
                match cell {
                    WALL => reward += -5.,
                    // WALL => reward += -20.,
                    ROBOT => reward += -3.,
                    DISCOVERED_EMPTY => reward += if num_visits > 5 { -5. } else { 1. },
                    // DISCOVERED_EMPTY => reward += -2.,
                    DISCOVERED_MINERAL => reward += if num_visits > 5 { -3. } else { 2. },
                    // DISCOVERED_MINERAL => reward += -1.,
                    JUST_DISCOVERED_EMPTY => reward += 2.,
                    JUST_DISCOVERED_MINERAL => reward += 30.,
                    _ => panic!("THIS CASE IS NOT COVERED"),
                }
            }

            // println!("reward: {}, position: {}", reward, new_position);

            return (new_position, vec![to_value(new_grid)], reward, false);
            /*****************************************/
        },
    );

    /************ UPDATING SCHEDULER *********/
    let robot_hive_mind = Rc::new(RefCell::new(
        load_q_table(&q_table_filepath).unwrap_or(HashMap::new()),
    ));

    scheduler.add_swarming_agents(
        10,
        None,
        BLUE,
        "robot_explorer",
        vec![to_value::<Vec<_>>(vec![0u32])],
        None,
        None,
        Some(0.01),
        // None,
        &agent_func,
        robot_hive_mind,
    );

    for _ in 0..4 {
        scheduler.train_agents(400);
        scheduler
            .env
            .set_persitent_elements(persistent_elements.clone());

        // Giving random position to agents
        for i in 0..scheduler.agents.len() {
            scheduler.agents[i].0 = scheduler.env.get_random_position();
        }

        // Reset visits
        scheduler.env.data.insert(VISITS, to_value(visits.clone()));
    }

    /*****************************************/

    scheduler
}

fn get_random_position(x: i32, y: i32, grid_size: &GridSize) -> (i32, i32) {
    let GridSize { width, heigth } = grid_size;

    let mut new_x = rand::random_range(x - 1..x + 2);
    let mut new_y = rand::random_range(y - 1..y + 2);

    while new_x >= *width as i32 || new_x < 0 || new_y >= *heigth as i32 || new_y < 0 {
        new_x = rand::random_range(x - 1..x + 2);
        new_y = rand::random_range(y - 1..y + 2);
    }

    (new_x, new_y)
}

fn generate_map(grid_size: GridSize, fill_ratio: f32, num_blob: i32) -> Vec<(i32, i32)> {
    assert!(fill_ratio < 1.0);

    let GridSize { width, heigth } = grid_size;
    let target_fill = ((grid_size.heigth * grid_size.width) as f32 * fill_ratio) as i32;
    let mut num_filled_cell = 0;
    let mut blob_positions: Vec<(i32, i32)> = (0..num_blob)
        .map(|_| {
            (
                rand::random_range(0..width) as i32,
                rand::random_range(0..heigth) as i32,
            )
        })
        .collect();

    // Filling position
    while num_filled_cell < target_fill {
        for i in 0..blob_positions.len() {
            let (x, y) = blob_positions[i];
            let (new_x, new_y) = get_random_position(x, y, &grid_size);
            if (x != new_x || y != new_y) && !blob_positions.contains(&(new_x, new_y)) {
                blob_positions.push((new_x, new_y));
                num_filled_cell += 1;
            }
        }
    }

    blob_positions
}

fn get_robot_state(current_pos: IVec2, env: &Env, fov: i32) -> Vec<((i32, i32), u32)> {
    let IVec2 {
        x: init_x,
        y: init_y,
    } = current_pos;

    let mut new_state: Vec<((i32, i32), u32)> = Vec::new();

    // let world: World = env.data.get(&WORLD).unwrap().eq_type();
    // let mut world = env.data.get_mut(&WORLD).unwrap().as_map_mut().unwrap();

    for x in init_x - fov..init_x + fov + 1 {
        for y in init_y - fov..init_y + fov + 1 {
            if env.position_inbound(IVec2 { x, y }) {
                // If cell is where the robot is
                if x == init_x && y == init_y {
                    new_state.push(((x, y), ROBOT));
                    continue;
                }

                // TODO if ally is on cell

                if let Some(color) = env.persistent_elements.get(&IVec2 { x, y }) {
                    match color {
                        &BASE_MINERAL => new_state.push(((x, y), JUST_DISCOVERED_MINERAL)),
                        &JUST_DISCOVERED_EMPTY_COLOR | &DISCOVERED_EMPTY_COLOR => {
                            new_state.push(((x, y), DISCOVERED_EMPTY))
                        }
                        &JUST_DISCOVERED_MINERAL_COLOR | &DISCOVERED_MINERAL_COLOR => {
                            new_state.push(((x, y), DISCOVERED_MINERAL))
                        }
                        color => println!("uncovered color {}", color.to_vec()),
                    }
                } else {
                    new_state.push(((x, y), JUST_DISCOVERED_EMPTY))
                }
            } else {
                // If out of bound it will be considered a wall (will also need to be implement in case wall are inside the map)
                new_state.push(((x, y), WALL));
            }
        }
    }

    new_state
}

fn update_visits(map: &mut HashMap<Value, Value>, key: Value) -> u32 {
    let val: u32 = map
        .get(&to_value(key.clone()))
        .unwrap_or(&mut Value::VU32(0))
        .eq_type();

    map.insert(to_value(key), to_value(val + 1));

    val + 1
}
