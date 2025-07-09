mod anim;
mod snacks;
mod snake;
mod stage;
mod state;
mod ui;

use bevy::prelude::*;
use anim::AnimPlugin;
use snacks::SnacksPlugin;
use snake::SnakePlugin;
use stage::StagePlugin;
use state::StatePlugin;
use ui::UIPlugin;

fn main() {
    App::new()
		.insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
		.insert_resource(AmbientLight {
			color: Color::default(),
			brightness: 450.0,
			affects_lightmapped_meshes: false,
		})
		.add_plugins((StatePlugin, SnakePlugin, StagePlugin, SnacksPlugin, UIPlugin, AnimPlugin))
		.add_plugins(DefaultPlugins)
		.run();
}
