mod score;
mod snacks;
mod snake;
mod stage;
mod state;

use bevy::prelude::*;
use score::ScorePlugin;
use snacks::SnacksPlugin;
use snake::SnakePlugin;
use stage::StagePlugin;
use state::StatePlugin;

fn main() {
    App::new()
		.insert_resource(ClearColor(Color::srgb(0.2, 0.2, 0.22)))
		.insert_resource(AmbientLight {
			color: Color::default(),
			brightness: 200.0,
			affects_lightmapped_meshes: false,
		})
		.add_plugins((ScorePlugin, SnacksPlugin, SnakePlugin, StagePlugin, StatePlugin))
		.add_plugins(DefaultPlugins)
		.run();
}
