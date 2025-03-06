use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::window::CursorGrabMode;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Fundamental".into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (
                rotate_sphere,
                camera_controller,
        ))
        .run();
}

#[derive(Component)]
struct RotatingSphere;
#[derive(Component)]
struct OrbitCamera {
    focus: Vec3,
    radius: f32,
    upside_down: bool,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self {
            focus: Vec3::ZERO,
            radius: 10.0,
            upside_down: false,
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        OrbitCamera {
            focus: Vec3::ZERO,
            radius: 0.0,
            upside_down: false,
        }
    ));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.2,
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(10.0, 10.0)),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.3, 0.5, 0.3),
            perceptual_roughness: 1.0,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(bevy::math::primitives::Sphere { radius: 0.5 })),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.8, 0.2, 0.3),
                metallic: 0.7,
                perceptual_roughness: 0.2,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        RotatingSphere,
    ));
}

fn rotate_sphere(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<RotatingSphere>>,
) {
    for mut transform in &mut query {
        let angle = time.elapsed_seconds() * PI / 2.0;
        let radius = 2.0;
        let x = radius * angle.cos();
        let z = radius * angle.sin();

        transform.translation.x = x;
        transform.translation.z = z;        
        transform.rotate_y(time.delta_seconds() * 2.0);
    }
}

fn camera_controller(
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&mut Transform, &mut OrbitCamera)>,
    mut window_query: Query<&mut Window>,
) {
    let mut window = window_query.single_mut();
    let mut rotation_move = Vec2::ZERO;
    let mut scroll = 0.0;
    let mut orbit_button_pressed = false;

    if input_mouse.pressed(MouseButton::Right) {
        for ev in ev_motion.read() {
            rotation_move += ev.delta;
        }
        orbit_button_pressed = true;
    }

    for ev in ev_scroll.read() {
        scroll = match ev.unit {
            MouseScrollUnit::Line => ev.y,
            MouseScrollUnit::Pixel => ev.y / 100.0,
        };
    }

    if input_mouse.just_pressed(MouseButton::Right) {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
    }
    if input_mouse.just_released(MouseButton::Right) {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }

    for (mut transform, mut orbit) in query.iter_mut() {
        if orbit_button_pressed {
            let sensitivity = 0.5;
            let delta_x = rotation_move.x * sensitivity;
            let delta_y = rotation_move.y * sensitivity;
            let mut position = transform.translation - orbit.focus;
            let mut rotation = transform.rotation;

            let yaw = Quat::from_rotation_y(-delta_x * 0.005);
            position = yaw * position;
            rotation = yaw * rotation;

            let pitch = Quat::from_rotation_x(-delta_y * 0.005);
            position = pitch * position;
            rotation = pitch * rotation;

            let up = rotation * Vec3::Y;
            orbit.upside_down = up.y <= 0.0;

            transform.translation = position + orbit.focus;
            transform.rotation = rotation;
        }
        if scroll != 0.0 {
            let scroll_sensitivity = 0.1;
            orbit.radius -= scroll * scroll_sensitivity;
            orbit.radius = orbit.radius.max(1.0).min(50.0);

            let direction = (transform.translation - orbit.focus).normalize();
            transform.translation = orbit.focus + direction * orbit.radius;
        }
        transform.look_at(orbit.focus, Vec3::Y);
    }
}
