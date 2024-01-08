//! A shader and a material that uses it.

use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
};

use bevy_terminal_shader::{TerminalMaterial, TerminalShaderPlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalShaderPlugin))
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<TerminalMaterial>>,
    _asset_server: Res<AssetServer>,
) {
    // cube
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        material: materials.add(
            // TerminalMaterial::default()
            TerminalMaterial::green(), // color: Color::BLUE,
                                       // color_texture: Some(asset_server.load("branding/icon.png")),
                                       // alpha_mode: AlphaMode::Blend,
        ),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
