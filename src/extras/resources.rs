use bevy::prelude::*;

#[derive(Resource)]
pub struct SimulationTime {
    pub elapsed: f32,
    pub speed_multiplier: f32,
}

#[derive(Resource)]
pub struct CameraController {
    pub sensitivity: f32,
    pub zoom_speed: f32,
}

#[derive(Resource)]
pub struct SelectionState {
    pub selected_entity: Option<Entity>,
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

