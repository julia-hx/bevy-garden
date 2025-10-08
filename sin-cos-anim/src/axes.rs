use bevy::prelude::*;
use crate::GameConfig;

pub struct AxesPlugin;

const COLOR_SCALAR:f32 = 0.24;

impl Plugin for AxesPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, spawn_axes);
	}
}

fn spawn_axes(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	config: Res<GameConfig>,
) {
	println!("spawn axes");

	if !config.render_axes { return; }

	let sphere_radius = config.global_scale * 0.042;
	let offset = config.global_scale;

	// x sin
	commands.spawn((
		Transform::from_xyz(-offset, 0.0, 0.0),
		Mesh3d(meshes.add(Sphere::new(sphere_radius))),
		MeshMaterial3d(materials.add(Color::linear_rgb(COLOR_SCALAR * 4.0, COLOR_SCALAR, COLOR_SCALAR))),
		// AnimComponent
	));

	// y sin
	commands.spawn((
		Transform::from_xyz(-offset, offset, 0.0),
		Mesh3d(meshes.add(Sphere::new(sphere_radius))),
		MeshMaterial3d(materials.add(Color::linear_rgb(COLOR_SCALAR, COLOR_SCALAR * 4.0, COLOR_SCALAR))),
		// AnimComponent
	));

	// z sin
	commands.spawn((
		Transform::from_xyz(-offset, 0.0, -offset),
		Mesh3d(meshes.add(Sphere::new(sphere_radius))),
		MeshMaterial3d(materials.add(Color::linear_rgb(COLOR_SCALAR, COLOR_SCALAR, COLOR_SCALAR * 4.0))),
		// AnimComponent
	));
}
