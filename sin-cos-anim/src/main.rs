mod anim;
mod axes;
mod cube;
mod gui;

use anim::AnimPlugin;
use axes::AxesPlugin;
use cube::CubePlugin;
use gui::GuiPlugin;

use bevy::prelude::*;

// design-time config
#[derive(Resource, Debug)]
pub struct GameConfig {
	pub render_axes: bool,
	pub render_cube: bool,
	pub render_gui: bool,

	pub global_scale: f32,
}

impl GameConfig {
	fn new() -> Self {
		Self {
			render_axes: true,
			render_cube: true,
			render_gui: true,

			global_scale: 4.0,
		}
	}
}

// realtime global data
#[derive(Resource, Debug, Default)]
pub struct GameData {
	pub x:f32,
	pub y:f32,
	pub z:f32,
}

impl GameData {
	fn new() -> Self {
		Self {
			x: 0.0,
			y: 0.0,
			z: 0.0,
		}
	}
}

fn main() {
    App::new()
		.insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
		.insert_resource(AmbientLight {
			color: Color::default(),
			brightness: 1000.0,
			affects_lightmapped_meshes: false,
		})
		.insert_resource(GameConfig::new())
		.insert_resource(GameData::new())
		.add_plugins(DefaultPlugins)
		.add_plugins((AnimPlugin, AxesPlugin, CubePlugin, GuiPlugin))
		.add_systems(Startup, set_stage)
		.run();
}

fn set_stage(
	mut commands: Commands,
) {
	commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 4_000_000.0,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(2.0, 4.0, 4.0),
    ));

	commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.0, 14.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
    ));
}
