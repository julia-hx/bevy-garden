use bevy::prelude::*;
use crate::GameConfig;
pub struct CubePlugin;

impl Plugin for CubePlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, spawn_cube);
	}
}

fn spawn_cube(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	config: Res<GameConfig>,
) {
	println!("spawn cube");

	if !config.render_cube { return; }

	let cube_scale = config.global_scale * 0.5;

	commands.spawn((
		Transform::from_xyz(0.0, 0.0, -config.global_scale * 0.5),
		Mesh3d(meshes.add(Cuboid::new(cube_scale, cube_scale, cube_scale))),
		MeshMaterial3d(materials.add(Color::linear_rgb(0.5, 0.5, 0.5))),
		// AnimComponent
	));
}
