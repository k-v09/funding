use bevy::prelude::*;

#[derive(Component)]
pub struct SpeedDisplay;

#[derive(Component)]
pub struct UiPanel {
    pub collapsed: bool,
}

#[derive(Component)]
pub struct Collapse;

#[derive(Component)]
pub struct Selectable;

#[derive(Component)]
pub struct ContentContainer;

#[derive(Component)]
pub struct AudioEmitter {
    pub frequency: f32, // in Hz
    pub amplitude: f32,
    pub phase: f32,
}

#[derive(Component)]
pub struct CameraOrbit {
    pub radius: f32,
    pub pitch: f32,
    pub yaw: f32,
}
#[derive(Component)]
pub struct ControlSection {
    pub section_type: SectionType,
    pub is_active: bool,
}
#[derive(PartialEq)]
pub enum SectionType {
    Speed,
}
