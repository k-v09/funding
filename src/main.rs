use bevy::prelude::*;
use bevy::math::primitives::{Sphere, Plane3d};
use bevy::input::mouse::MouseMotion;
use bevy::input::keyboard::KeyCode;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
//use bevy::window::PrimaryWindow;

#[derive(Component)]
struct AudioEmitter {
    frequency: f32, // in Hz
    amplitude: f32,
    phase: f32,
}
#[derive(Resource)]
struct SimulationTime {
    elapsed: f32,
    speed_multiplier: f32,
}
#[derive(Resource)]
struct CameraController {
    sensitivity: f32,
    zoom_speed: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .insert_resource(CameraController { sensitivity: 0.005, zoom_speed: 0.5, })
        .insert_resource(SimulationTime { elapsed: 0.0, speed_multiplier: 0.1 })
        .add_systems(Startup, setup)
        .add_systems(Update, (camera_controller, update_sim, control_system,))
        .run();
}

fn update_sim(
    time: Res<Time>,
    mut sim_time: ResMut<SimulationTime>,
    mut query: Query<(&mut Transform, &AudioEmitter)>,
) {
    sim_time.elapsed += time.delta_seconds() * sim_time.speed_multiplier;

    for (mut transform, emitter) in query.iter_mut() {
        let wave = ((emitter.frequency * sim_time.elapsed * std::f32::consts::TAU) + emitter.phase).sin();
        let scale = 1.0 + wave * emitter.amplitude;
        transform.scale = Vec3::splat(scale);
    }
}

fn control_system(
    mut contexts: EguiContexts,
    mut sim_time: ResMut<SimulationTime>,
) {
    egui::Window::new("Simulation Controls")
        .anchor(egui::Align2::LEFT_TOP, egui::Vec2::new(10.0, 10.0))
        .default_width(200.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.label("Simulation Speed");
            ui.add(egui::Slider::new(&mut sim_time.speed_multiplier, 0.0..=1.0)
                .text("Speed")
                .suffix("x"));
        });
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        CameraOrbit {
            radius: 5.0,
            pitch: 0.2,
            yaw: 0.0,
        },
    ));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Am chord
    let emitter_configs = [
        (440.0, Color:: RED, 0.0),     // 0
        (523.25, Color::GREEN, 2.094), // 2pi/3
        (660.0, Color::BLUE, 4.189),   // 4pi/3
    ];

    for (i, (frequency, color, phase)) in emitter_configs.iter().enumerate() {
        let angle = (i as f32 / emitter_configs.len() as f32) * std::f32::consts::TAU;
        let radius = 2.0;
        let x = radius * angle.cos();
        let z = radius * angle.sin();

        commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Sphere::new(0.5).mesh()),
                    material: materials.add(StandardMaterial {
                        base_color: *color,
                        emissive: *color * 0.2,
                        ..default()
                    }),
                    transform: Transform::from_xyz(x, 0.5, z),
                    ..default()
                },
                AudioEmitter {
                    frequency: *frequency,
                    amplitude: 0.3, // scale range of 0.7 to 1.3
                    phase: *phase,
                },
        ));
    }

    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh()),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.3, 0.3, 0.3),
            ..default()
        }),
        ..default()
    });
}

#[derive(Component)]
struct CameraOrbit {
    radius: f32,
    pitch: f32,
    yaw: f32,
}

fn camera_controller(
    mut mouse_motion: EventReader<MouseMotion>,
    keyboard: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<MouseButton>>,
    controller: Res<CameraController>,
    mut query: Query<(&mut Transform, &mut CameraOrbit), With<Camera>>,
) {
    let Ok((mut transform, mut orbit)) = query.get_single_mut() else { return };
    if buttons.pressed(MouseButton::Right) {
        for event in mouse_motion.read() {
            orbit.yaw -= event.delta.x * controller.sensitivity;
            orbit.pitch -= event.delta.y * controller.sensitivity;
            // Prevent camera flipping
            orbit.pitch = orbit.pitch.clamp(-1.5, 1.5);
        }
    }

    if keyboard.pressed(KeyCode::ArrowUp) {
        orbit.radius -= controller.zoom_speed;
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        orbit.radius += controller.zoom_speed;
    }
    orbit.radius = orbit.radius.clamp(2.0, 20.0);

    let new_transform = Transform::from_xyz(
        orbit.radius * orbit.yaw.cos() * orbit.pitch.cos(),
        orbit.radius * orbit.pitch.sin(),
        orbit.radius * orbit.yaw.sin() * orbit.pitch.cos(),
    ).looking_at(Vec3::ZERO, Vec3::Y);

    *transform = new_transform;
}

/*
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
*/
