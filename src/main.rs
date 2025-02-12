use bevy::{
    prelude::*,
    input::{mouse::MouseMotion, keyboard::KeyCode},
    math::DVec3,
};
mod extras {
    pub mod components;
    pub mod resources;
}
use extras::components::*;
use extras::resources::*;
/*
use bevy::render::render_resource::*;
use bevy::asset::Handle;
use bevy::render::renderer::{RenderDevice, RenderQueue};
use wgpu::{Device, Queue, CommandEncoder};
//use bevy::render::render_resource::{ShaderStages, Shader, ShaderType};
//use bevy::window::PrimaryWindow;
*/

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
    selection_state: Res<SelectionState>,
) {
    if selection_state.selected_entity.is_some() { return }
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
    selection_state: Res<SelectionState>,
) {
    if selection_state.selected_entity.is_some() { return }
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
        .insert_resource(SelectionState { selected_entity: None })
        .add_systems(Startup, (
                setup,
                setup_ui,
            ))
        .add_systems(Update, (
//                run_shader,
                camera_controller,
                update_sim,
                object_selection,
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
                Selectable,
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

fn object_selection(
    _commands: Commands,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window_query: Query<&Window>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut selection_state: ResMut<SelectionState>,
    selectable_query: Query<(Entity, &GlobalTransform), With<Selectable>>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) { return }
    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();

    if let Some(cursor_position) = window.cursor_position() {
        if let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) {
            let mut closest_hit: Option<(Entity, f32)> = None;
            for (entity, transform) in selectable_query.iter() {
                let sphere_center = transform.translation();
                let sphere_radius = 0.5; // gotta double check sphere size

                // Intersection check
                let sphere_center_d: DVec3 = DVec3::new(
                    sphere_center.x as f64,
                    sphere_center.y as f64,
                    sphere_center.z as f64,
                );
                let ray_origin_d = DVec3::new(
                    ray.origin.x as f64,
                    ray.origin.y as f64,
                    ray.origin.z as f64,
                );
                let offset_d = ray_origin_d - sphere_center_d;
                let dir_vec = ray.direction.as_dvec3();
                let a = dir_vec.dot(dir_vec);
                let b = 2.0 * dir_vec.dot(offset_d);
                let c = offset_d.dot(offset_d) - (sphere_radius * sphere_radius) as f64;
                let discr = b * b - 4.0 * a * c;
                if discr >= 0.0 {
                    let distance = ((-b - discr.sqrt()) / (2.0 * a)) as f32;
                    if distance >= 0.0 {
                        match closest_hit {
                            None => closest_hit = Some((entity, distance)),
                            Some((_, best_dist)) if distance < best_dist => {
                                closest_hit = Some((entity, distance))
                            }
                            _ => {}
                        }
                    }
                }
            }
            selection_state.selected_entity = closest_hit.map(|(entity, _)| entity);
        }
    }
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
