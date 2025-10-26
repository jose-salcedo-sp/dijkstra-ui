use std::collections::HashSet;
use std::sync::{Arc, RwLock};

use bevy::color::palettes::css::{GREEN, RED, WHITE, YELLOW};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

mod graph;
use graph::{Edge, Graph};

#[derive(Component)]
struct MainCamera;

#[derive(Component, Clone, Debug)]
struct Node {
    position: Vec2,
    r: f32,
    id: usize,
}

#[derive(Component)]
struct WrapperGraph(Arc<RwLock<Graph>>);

#[derive(Component)]
struct SelectedNode {
    id: Option<usize>,
}

#[derive(Component, Debug)]
struct StartNode {
    id: Option<usize>,
}

#[derive(Component, Debug)]
struct GoalNode {
    id: Option<usize>,
}

#[derive(Component)]
struct SelectedRing;

#[derive(Component, Clone, Debug)]
struct NodeMat(Handle<ColorMaterial>);

#[derive(Component, Clone)]
struct EdgeMat(Handle<ColorMaterial>);

#[derive(Component)]
struct Background;

#[derive(Component)]
struct EdgeVisual {
    a: usize,
    b: usize,
}

#[derive(Resource, Default)]
struct HighlightedEdges(HashSet<(usize, usize)>);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .init_resource::<HighlightedEdges>()
        .add_systems(Update, (handle_click, handle_keyboard_input))
        .add_systems(
            Update,
            (
                add_node_visuals,
                update_selected_ring,
                update_node_colors,
                update_edge_colors,
            ),
        )
        .run();
}

fn ord(a: usize, b: usize) -> (usize, usize) {
    if a < b { (a, b) } else { (b, a) }
}

fn setup(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Camera2d,
        MainCamera,
        SelectedNode { id: None },
        StartNode { id: None },
        GoalNode { id: None },
        WrapperGraph(Arc::new(RwLock::new(Graph { nodes: vec![] }))),
    ));

    let win = windows.single().expect("primary window");
    let (w, h) = (win.width(), win.height());
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(w, h))),
        MeshMaterial2d(materials.add(Color::BLACK)),
        Transform::from_translation(Vec3::new(0.0, 0.0, -100.0)),
        Background,
    ));
}

fn add_node_visuals(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    q_added: Query<(Entity, &Node), Added<Node>>,
) {
    for (e, node) in q_added.iter() {
        let mat = materials.add(ColorMaterial::from(Color::from(WHITE)));

        commands.entity(e).insert((
            Mesh2d(meshes.add(Circle::new(node.r))),
            MeshMaterial2d(mat.clone()),
            NodeMat(mat),
            Transform::from_translation(Vec3::new(node.position.x, node.position.y, 0.0)),
            Text2d::new((b'A' + node.id as u8) as char),
            TextColor(Color::BLACK),
        ));
    }
}

fn update_selected_ring(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    q_sel: Query<&SelectedNode, (With<MainCamera>, Changed<SelectedNode>)>,
    q_nodes: Query<&Node>,
    q_old: Query<Entity, With<SelectedRing>>,
) {
    if q_sel.is_empty() {
        return;
    }

    for e in q_old.iter() {
        commands.entity(e).despawn();
    }

    let selected_id = q_sel.single().unwrap().id;
    let Some(id) = selected_id else {
        return;
    };
    let Some(node) = q_nodes.iter().find(|n| n.id == id) else {
        return;
    };

    commands.spawn((
        Mesh2d(meshes.add(Annulus::new(node.r + 5.0, node.r + 8.0))),
        MeshMaterial2d(materials.add(Color::from(RED))),
        Transform::from_translation(Vec3::new(node.position.x, node.position.y, 1.0)),
        SelectedRing,
    ));
}

fn update_node_colors(
    q_flags: Query<
        (&StartNode, &GoalNode),
        (
            With<MainCamera>,
            Or<(Changed<StartNode>, Changed<GoalNode>)>,
        ),
    >,
    q_nodes: Query<(&Node, &NodeMat)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) -> Result<()> {
    if q_flags.is_empty() {
        return Ok(());
    }
    let (start, goal) = q_flags.single()?;

    for (node, NodeMat(handle)) in q_nodes.iter() {
        if let Some(m) = materials.get_mut(handle) {
            m.color = if start.id == Some(node.id) {
                Color::from(GREEN)
            } else if goal.id == Some(node.id) {
                Color::from(YELLOW)
            } else {
                Color::from(WHITE)
            };
        }
    }

    return Ok(());
}

fn update_edge_colors(
    highlights: Res<HighlightedEdges>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut edges: Query<(&EdgeVisual, &EdgeMat)>,
) {
    let on = Color::from(bevy::color::palettes::css::AQUA);
    let off = Color::from(bevy::color::palettes::css::WHITE);

    for (ev, EdgeMat(h)) in &mut edges {
        let target = if highlights.0.contains(&ord(ev.a, ev.b)) { on } else { off };
        if let Some(m) = materials.get_mut(h) {
            m.color = target;
        }
    }
}

fn cursor_world(
    windows: &Query<&Window, With<PrimaryWindow>>,
    cams: &Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) -> Option<Vec2> {
    let window = windows.single().ok()?;
    let cursor = window.cursor_position()?;
    let (camera, cam_xform) = cams.single().ok()?;
    Some(
        camera
            .viewport_to_world(cam_xform, cursor)
            .ok()?
            .origin
            .truncate(),
    )
}

fn handle_keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    wrapper_graph: Query<&WrapperGraph, With<MainCamera>>,
    mut selected_node: Query<&mut SelectedNode, With<MainCamera>>,
    mut start_node: Query<&mut StartNode, With<MainCamera>>,
    mut goal_node: Query<&mut GoalNode, With<MainCamera>>,
    mut highlights: ResMut<HighlightedEdges>,
) -> Result<()> {
    if keys.just_pressed(KeyCode::KeyP) {
        let wg = wrapper_graph.single()?;
        let graph = wg.0.read().unwrap();
        highlights.0.clear();

        let (Some(start_node_id), Some(goal_node_id)) =
            (start_node.single()?.id, goal_node.single()?.id)
        else {
            println!("Missing starting or goal node!");
            return Ok(());
        };
        let Some((length, path)) = graph.shortest_path(start_node_id, goal_node_id) else {
            println!("No current available path");
            return Ok(());
        };

        println!("Path length: {}, Path: {}", length, Graph::fmt_path(&path));

        for w in path.windows(2) {
            highlights.0.insert(ord(w[0], w[1]));
        }
        return Ok(());
    }

    let mut selected_id = selected_node.single_mut()?.id;
    if selected_id.is_none() {
        return Ok(());
    }
    let Some(id) = selected_id.take() else {
        return Ok(());
    };

    for key in keys.get_just_pressed() {
        match key {
            KeyCode::KeyS => {
                start_node.single_mut()?.id = Some(id);
                if goal_node.single()?.id == Some(id) {
                    goal_node.single_mut()?.id = None;
                }
            }
            KeyCode::KeyG => {
                goal_node.single_mut()?.id = Some(id);
                if start_node.single()?.id == Some(id) {
                    start_node.single_mut()?.id = None;
                }
            }
            _ => { /* unhandled keycode */ }
        }
    }

    return Ok(());
}

fn clicked_node_id(nodes: &Query<&Node>, world: Vec2) -> Option<usize> {
    for node in nodes {
        if (world - node.position).length() < node.r {
            return Some(node.id);
        }
    }
    None
}

fn handle_click(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cams: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    nodes: Query<&Node>,
    mut wrapper_graph: Query<&mut WrapperGraph, With<MainCamera>>,
    mut selected_node: Query<&mut SelectedNode, With<MainCamera>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) -> Result<()> {
    if !buttons.just_pressed(MouseButton::Left) {
        return Ok(());
    }
    let Some(world) = cursor_world(&windows, &cams) else {
        return Ok(());
    };
    let clicked = clicked_node_id(&nodes, world);

    let wg = wrapper_graph.single_mut()?;
    let mut graph = wg.0.write().unwrap();

    if let Some(clicked_node_id) = clicked {
        if let Some(prev_selected_node_id) = selected_node.single_mut()?.id.take() {
            let clicked_node = nodes.iter().find(|n| n.id == clicked_node_id).unwrap();

            if graph.nodes[clicked_node_id]
                .iter()
                .any(|n| n.node == prev_selected_node_id)
            {
                selected_node.single_mut()?.id = Some(clicked_node_id);
                return Ok(());
            }

            let prev_selected_node = nodes
                .iter()
                .find(|n| n.id == prev_selected_node_id)
                .unwrap();
            let d = clicked_node.position - prev_selected_node.position;
            let len = d.length();
            let angle = d.y.atan2(d.x);
            let mid = (clicked_node.position + prev_selected_node.position) * 0.5;
            let thickness = 2.0;

            let cost = len as usize;
            graph.nodes[clicked_node_id].push(Edge {
                node: prev_selected_node_id,
                cost,
            });
            graph.nodes[prev_selected_node_id].push(Edge {
                node: clicked_node_id,
                cost,
            });

            let mat = materials.add(ColorMaterial::from(Color::WHITE));

            commands.spawn((
                Mesh2d(meshes.add(Rectangle::new(len, thickness))),
                MeshMaterial2d(mat.clone()),
                EdgeMat(mat),
                Transform {
                    translation: Vec3::new(mid.x, mid.y, -10.0),
                    rotation: Quat::from_rotation_z(angle),
                    ..Default::default()
                },
                EdgeVisual {
                    a: prev_selected_node_id,
                    b: clicked_node_id,
                },
            ));

            selected_node.single_mut()?.id = None;
            return Ok(());
        }
        selected_node.single_mut()?.id = Some(clicked_node_id);
    } else {
        selected_node.single_mut()?.id = None;
        graph.nodes.push(Vec::new());
        let new_id = graph.nodes.len() - 1;

        commands.spawn(Node {
            position: world,
            r: 20.0,
            id: new_id,
        });
    }

    return Ok(());
}
