//! Shows how to render a polygonal [`Mesh`], generated from a [`Quad`] primitive, in a 2D scene.

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_terminal_shader::{TerminalMaterial, TerminalShaderPlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalShaderPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<TerminalMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::new(50., 100.)).into())
            .into(),
        material: materials.add(TerminalMaterial::green()),
        transform: Transform::from_translation(Vec3::new(50., 0., 0.)),
        ..default()
    });
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default().with_scale(Vec3::splat(2.)),
        material: materials.add(TerminalMaterial::green()),
        ..default()
    });
}
