# Dijkstra Path Visualizer (Bevy)

An interactive visualization tool for **Dijkstra’s shortest path algorithm** built using the [Bevy](https://bevyengine.org) game engine in Rust.
This project lets users **create graph nodes and edges**, set **start and goal nodes**, and visualize the computed **shortest path** in real time.

---

## Features

- **Dynamic Graph Creation** — Click anywhere to spawn nodes. Connect nodes by selecting two in succession.
- **Start/Goal Selection** — Press `S` or `G` while a node is selected to mark it as the start or goal node.
- **Path Computation** — Press `P` to compute and display the shortest path between the start and goal using Dijkstra’s algorithm.
- **Visual Feedback**
    - Start node → **Green**
    - Goal node → **Yellow**
    - Selected node → **Red ring**
    - Path edges → **Aqua**

---

## How It Works

The app uses **Bevy’s ECS (Entity–Component–System)** model to separate logic cleanly:

- **Entities** represent graph components such as nodes, edges, and UI elements.
- **Components** store data like positions, materials, and identifiers.
- **Systems** handle input events, graph updates, and rendering logic.

The underlying pathfinding is handled by the `Graph` and `Edge` structures in `graph.rs`, which implement **Dijkstra’s algorithm** using a binary heap for efficient shortest-path computation.

---

## Controls

| Action                | Input                             |
| --------------------- | --------------------------------- |
| Spawn Node            | Left-click on empty space         |
| Connect Nodes         | Left-click one node, then another |
| Select Node           | Left-click on an existing node    |
| Set Start Node        | Press `S` with a node selected    |
| Set Goal Node         | Press `G` with a node selected    |
| Compute Shortest Path | Press `P`                         |

---

## Project Structure
```

src/
├── main.rs # Bevy app setup, ECS systems, and UI logic
└── graph.rs # Graph data structure and Dijkstra’s algorithm

````

---

## Running the Project

### Prerequisites
- **Rust** (latest stable)
- **Bevy** dependencies (installed automatically via Cargo)

### Run
```bash
cargo run
````
