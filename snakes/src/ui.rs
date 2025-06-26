use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, init_ui_fields);
	}
}

#[derive(Component, Copy, Clone)]
#[require(Node)]
struct TextElement {
	id: &'static str,

}

impl TextElement {
	fn new(id: &'static str) -> Self {
		Self {
			id,
		}
	}
}

fn init_ui_fields(mut commands: Commands) {
	commands.spawn((
		TextElement::new("test"),
		Text::new("hello bevy!"),
		Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },

	));
}
