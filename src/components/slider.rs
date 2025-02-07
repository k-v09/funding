use bevy::prelude::*;

#[derive(Component)]
pub struct Slider {
    pub min: f32,
    pub max: f32,
    pub value: f32,
    pub width: f32,
}

impl Slider {
    pub fn new(min: f32, max: f32, initial: f32) -> Self {
        Self {
            min,
            max,
            value: initial.clamp(min, max),
        }
    }
}

pub fn spawn_slider(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px(200.0),
                height: Val::Px(20.0),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::GRAY),
            ..default()
        },
        Slider::new(0.0, 100.0, 50.0),
    ))
    .with_children(|parent| {
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Px(20.0),
                height: Val::Px(20.0),
                position_type: PositionType::Absolute,
                left: Val::Px(50.0), // Start position
                ..default()
            },
            background_color: BackgroundColor(Color::WHITE),
            ..default()
        });
    });
}

pub fn update_slider(
    mut slider_query: Query<(&mut Slider, &mut Style)>,
    mut cursor_moved: EventReader<CursorMoved>,
) {
    for event in cursor_moved.iter() {
        for (mut slider, mut style) in slider_query.iter_mut() {
            let new_x = event.position.x.clamp(slider.min, slider.max);
            slider.value = new_x;
            style.left = Val::Px(new_x);
        }
    }
}

