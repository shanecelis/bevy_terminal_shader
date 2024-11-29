//! Shows the terminal material rendered on a quad.

use bevy::prelude::*;
use bevy_terminal_shader::{TerminalMaterial, TerminalShaderPlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalShaderPlugin))
        .add_systems(Startup, setup)
        .run();
}

/// Setup a quad and camera.
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<TerminalMaterial>>,
) {
    commands.spawn(Camera2d::default());

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(1300., 800.).mesh())),
        MeshMaterial2d(materials.add(TerminalMaterial::green())),
    ));
}
