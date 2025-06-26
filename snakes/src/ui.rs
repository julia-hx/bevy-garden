use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, test_text);
	}
}

fn test_text(mut commands: Commands){
	commands.spawn((
		Text::new("hello bevy!"),
	));
}
