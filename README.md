# MASim

Multi-agent system simulator (MASim). Allow users to create and configure environments, place agents in those environments, and observe how they interact or evolve over time.
| ![runner_demo](./pictures/runner_demo.gif) | ![demo_mining_bot](./pictures/demo_mining_bots.gif) |
|---|---|

## Table of content:

- [Installation](#installation)
- [Usage](#usage)
- [Examples](#examples)

## Installation

1. **Install [Rust](https://www.rust-lang.org/fr/tools/install)**

2. **Clone the repo:**

```sh
git clone https://github.com/JoyousOne/MASim.git
cd MASim
```

3. **Dependencies**: Will be automatically installed when running cargo. To consult the dependencies, see [Cargo.toml](/Cargo.toml).

## Usage

1. **In `src/playground` add the file you want to play with.**

2. **Import in `src/main.rs` and tinker with it as you which.**

3. **Build & Run**:

```sh
cargo run
```

## Examples

### Runner

In this example, runners spawn in random positions. Their objective is to reach the `goal`, which is represented by a green cell. They start with a trained Q-table, after which they continue learning independently.

![runner_demo](./pictures/runner_demo.gif)

### Mining bots

In this example, multiple bots explore a 2D map to find mineral veins. They form a swarm in the sense that they coordinate or share information. Concretely, they all reference and update a shared Q-table. This means that whenever any single bot has a learning update (e.g., it explores new territory, encounters a mineral, or hits a wall), that update modifies the Q-values for all bots. In other words, they are learning from collective experience, which can speed up learning if the environment and tasks are similar for all agents.

![demo_mining_bot](./pictures/demo_mining_bots.gif)
