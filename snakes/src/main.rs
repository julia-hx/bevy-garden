mod snacks;
mod snake;
mod stage;
mod state;

use bevy::prelude::*;
use snacks::SnacksPlugin;
use snake::SnakePlugin;
use stage::StagePlugin;
use state::StatePlugin;

fn main() {
    App::new()
		.insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
		.insert_resource(AmbientLight {
			color: Color::default(),
			brightness: 250.0,
			affects_lightmapped_meshes: false,
		})
		.add_plugins((SnakePlugin, StagePlugin, SnacksPlugin, StatePlugin)) // using events between plugins, changing the order here seems to break things?
		.add_plugins(DefaultPlugins)
		.run();
}
