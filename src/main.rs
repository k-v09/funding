use bevy::prelude::*;
//use bevy::math::primitives::{Sphere, Plane3d};
use bevy::input::mouse::MouseMotion;
use bevy::input::keyboard::KeyCode;
use bevy::ui::Interaction;
/*
use bevy::render::render_resource::*;
use bevy::asset::Handle;
use bevy::render::renderer::{RenderDevice, RenderQueue};
use wgpu::{Device, Queue, CommandEncoder};
//use bevy::render::render_resource::{ShaderStages, Shader, ShaderType};
//use bevy::window::PrimaryWindow;
*/

#[derive(Component)]
struct SpeedDisplay;
#[derive(Component)]
struct UiPanel {
    collapsed: bool,
}
#[derive(Component)]
struct Collapse;
#[derive(Component)]
struct ContentContainer;
#[derive(Component)]
struct AudioEmitter {
    frequency: f32, // in Hz
    amplitude: f32,
    phase: f32,
}
#[derive (Component)]
struct CameraOrbit {
    radius: f32,
    pitch: f32,
    yaw: f32,
}
#[derive(Component)]
struct ControlSection {
    section_type: SectionType,
    is_active: bool,
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
#[derive(PartialEq)]
enum SectionType {
    Speed,
}
/*
#[derive(Resource)]
struct ComputePipeline {
    pipeline: CachedComputePipelineId,
    bind_group: BindGroup,
    buffer: Buffer,
}
#[derive(Resource)]
struct WaveMaterial {
    shader: Handle<Shader>,
}*/

fn setup_ui(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px(200.0),
                height: Val::Percent(90.0),
                position_type: PositionType::Absolute,
                left: Val::Px(20.0),
                top: Val::Px(20.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::rgba(0.2, 0.2, 0.2, 0.8).into(),
            ..default()
        },
        UiPanel { collapsed: false },
    ))
    .with_children(|parent| {
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(30.0),
                    height: Val::Px(30.0),
                    margin: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::NONE.into(),
                ..default()
            },
            Collapse,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                ">",
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..default()
                }
            ));
        });
        parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    padding: UiRect::horizontal(Val::Px(10.0)),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            ContentContainer,
        ))
        .with_children(|parent| {
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        padding: UiRect::all(Val::Px(10.0)),
                        margin: UiRect::bottom(Val::Px(10.0)),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::GRAY.into(),
                    ..default()
                },
                ControlSection {
                    section_type: SectionType::Speed,
                    is_active: false,
                },
                SpeedDisplay,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Speed",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    }
                ));
                parent.spawn(TextBundle::from_section(
                    "0.01",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    }
                ));
            });
        });
    });
}

fn hsa(
    mut interaction_query: Query<
        (&Interaction, &mut ControlSection, &mut BackgroundColor),
        (Changed<Interaction>, With<SpeedDisplay>)
    >,
) {
    for (interaction, mut section, mut bg_color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                section.is_active = !section.is_active;
                *bg_color = if section.is_active {
                    Color::rgb(0.3, 0.4, 0.6).into()
                } else {
                    Color::GRAY.into()
                };
            }
            _=> {}
        }
    }
}

fn update_collapse(
    interaction_query: Query<
        (&Interaction, &Children),
        (Changed<Interaction>, With<Collapse>)
    >,
    mut text_query: Query<&mut Text>,
    mut panel_query: Query<&mut UiPanel>,
) {
    for (interaction, children) in interaction_query.iter() {
        if let Interaction::Pressed = interaction {
            if let Ok(mut panel) = panel_query.get_single_mut() {
                panel.collapsed = !panel.collapsed;
                for & child in children {
                    if let Ok(mut text) = text_query.get_mut(child) {
                        text.sections[0].value = if panel.collapsed { "^".to_string() } else { ">".to_string() };
                    }
                }
            }
        }
    }
}

fn content_visibility(
    mut panel_query: Query<(&UiPanel, &mut Style)>,
    mut content_query: Query<&mut Visibility, With<ContentContainer>>,
) {
    let (panel, mut style) = panel_query.single_mut();
    style.height = if panel.collapsed {
        Val::Px(40.0)
    } else {
        Val::Percent(90.0)
    };
    for mut visibility in content_query.iter_mut() {
        *visibility = if panel.collapsed {
            Visibility::Hidden
        } else {
            Visibility::Visible
        };
    }
}

fn usd(
    sim_time: Res<SimulationTime>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut section_query: Query<(&ControlSection, &Children, &mut BackgroundColor), With<SpeedDisplay>>,
    mut text_query: Query<&mut Text>,
    panel_query: Query<&UiPanel>,
) {
    let is_collapsed = panel_query.single().collapsed;
    let (section, children, mut bg_color) = section_query.single_mut();
    let is_active = !is_collapsed && section.is_active && (keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::ArrowRight));

    *bg_color = if section.is_active {
        if is_active {
            Color::rgb(0.4, 0.6, 0.8).into()
        } else {
            Color::rgb(0.3, 0.4, 0.6).into()
        }
    } else {
        Color::GRAY.into()
    };

    if let Some(&last_child) = children.last() {
        if let Ok(mut text) = text_query.get_mut(last_child) {
            text.sections[0].value = format!("{:.3}", sim_time.speed_multiplier);
        }
    }
}

fn ssi(
    mut sim_time: ResMut<SimulationTime>,
    keyboard: Res<ButtonInput<KeyCode>>,
    panel_query: Query<&UiPanel>,
    section_query: Query<&ControlSection, With<SpeedDisplay>>,
) {
    let is_collapsed = panel_query.single().collapsed;
    if is_collapsed { return }
    let speed_section = section_query.single();
    if !speed_section.is_active { return }

    let speed_delta = 0.001;
    if keyboard.pressed(KeyCode::ArrowRight) {
        sim_time.speed_multiplier = (sim_time.speed_multiplier + speed_delta).min(0.1);
    }
    if keyboard.pressed(KeyCode::ArrowLeft) {
        sim_time.speed_multiplier = (sim_time.speed_multiplier - speed_delta).max(0.0);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(CameraController { sensitivity: 0.005, zoom_speed: 0.5, })
        .insert_resource(SimulationTime { elapsed: 0.0, speed_multiplier: 0.01 })
        .add_systems(Startup, (
                setup,
                setup_ui,
            ))
        .add_systems(Update, (
//                run_shader,
                camera_controller,
                update_sim,
                hsa,
                ssi,
                usd,
                update_collapse,
                content_visibility,
            ))
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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
//    mut shaders: ResMut<Assets<Shader>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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
fn setup(
    mut commands: Commands,
    mut pipeline_cache: ResMut<PipelineCache>,
    mut shaders: ResMut<Assets<Shader>>,
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
) {
    let shader_handle = shaders.add(Shader::from_wgsl(include_str!("./shaders/wave-shader.wgsl")));
    
    let buffer = render_device.create_buffer(&BufferDescriptor {
        label: Some("Compute Amplitude Buffer"),
        size: (std::mem::size_of::<f32>() * 64) as u64,
        usage: BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });

    let bind_group_layout = render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("Compute Bind Group Layout"),
        entries: &[BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Storage { read_only: false },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    });

    let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
        label: Some("Compute Bind Group"),
        layout: &bind_group_layout,
        entries: &[BindGroupEntry {
            binding: 0,
            resource: buffer.as_entire_binding(),
        }],
    });

    let pipeline_layout = render_device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some("Compute Pipeline Layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let pipeline = pipeline_cache.create_compute_pipeline(ComputePipelineDescriptor {
        label: Some("Compute Pipeline"),
        layout: Some(pipeline_layout),
        shader: shader_handle,
        shader_defs: vec![],
        entry_point: "main".into(),
    });

    commands.insert_resource(ComputePipeline { pipeline, bind_group, buffer });
}

fn run_shader(
    pipeline_cache: Res<PipelineCache>,
    pipeline: Res<ComputePipeline>,
    mut encoder: ResMut<CommandEncoder>,
) {
    if let Some(pipeline) = pipeline_cache.get_compute_pipeline(pipeline.pipeline) {
        let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor::default());
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, &pipeline.bind_group, &[]);
        pass.dispatch_workgroups(1,1,1);
    }
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
*/
