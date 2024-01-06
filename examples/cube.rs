use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use bevy_video_glitch::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, VideoGlitchPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate, update_settings))
        .run();
}

/// Set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0))
                .looking_at(Vec3::default(), Vec3::Y),
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::WHITE),
                ..default()
            },
            ..default()
        },
        // Add the setting to the camera.
        //
        // This component is also used to determine on which camera to run the
        // post processing effect.
        VideoGlitchSettings {
            intensity: 0.02,
            color_aberration: Mat3::from_cols_array(&[0.5, 0.5, 0.0,
                                                      0.0, 0.5, 0.5,
                                                      0.5, 0.0, 0.5]).transpose(),
            ..default()
        },
    ));

    // cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Rotates,
    ));
    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
        ..default()
    });
}

#[derive(Component)]
struct Rotates;

/// Rotates any entity around the x and y axis
fn rotate(time: Res<Time>, mut query: Query<&mut Transform, With<Rotates>>) {
    for mut transform in &mut query {
        transform.rotate_x(0.55 * time.delta_seconds());
        transform.rotate_z(0.15 * time.delta_seconds());
    }
}

// Change the intensity over time to show that the effect is controlled from the main world
fn update_settings(mut settings: Query<&mut VideoGlitchSettings>, time: Res<Time>) {
    for mut setting in &mut settings {
        let mut intensity = time.elapsed_seconds();
        // Make it loop periodically.
        intensity = intensity.sin();
        // Remap it to 0..1 because the intensity can't be negative.
        intensity = intensity * 0.5 + 0.5;
        // Set the intensity.
        //
        // This will then be extracted to the render world and uploaded to the
        // gpu automatically by the [`UniformComponentPlugin`].
        setting.intensity = intensity;
    }
}
