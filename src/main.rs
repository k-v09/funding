use bevy::prelude::*;
use bevy::math::primitives::Sphere;
use bevy::math::primitives::Plane3d;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate_camera, update_audio_visualization))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Audio emitter visualization (placeholder sphere)
    commands.spawn(PbrBundle {
        mesh: meshes.add(Sphere::new(0.5).mesh()),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.8, 0.2, 0.2),
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });

    // Ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh()),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.3, 0.3, 0.3),
            ..default()
        }),
        ..default()
    });
}

fn rotate_camera(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    for mut transform in &mut query {
        transform.rotate_around(
            Vec3::ZERO,
            Quat::from_rotation_y(0.3 * time.delta_seconds()),
        );
    }
}

fn update_audio_visualization() {
    // TODO: Add audio processing and visualization updates
}
