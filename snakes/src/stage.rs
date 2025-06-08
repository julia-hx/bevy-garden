use bevy::{prelude::*, text::cosmic_text::ttf_parser::ankr::Point};

pub struct StagePlugin;

impl Plugin for StagePlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, init_stage);
		// app.add_systems(Update, ())
	}
}

#[derive(Bundle)]
struct StageBundle {
	stage_manager: StageManager,
	camera: Camera3d,
	transform: Transform,
	spotlight: PointLight,
}

#[derive(Component)]
struct StageManager {
	current_stage_id: u32,
}

impl StageManager {
	fn new() -> Self {
		Self { current_stage_id: 0 }
	}
}

fn init_stage(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>
) {
	commands.spawn(StageBundle{
		stage_manager: StageManager::new(),
		camera: Camera3d::default(),
		transform: Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
		spotlight: PointLight { shadows_enabled: true, ..default() },
	});

	// test cube
	commands.spawn((
		Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
		MeshMaterial3d(materials.add(Color::srgb_u8(220, 220, 220))),
		Transform::from_xyz(0.0, 0.0, 0.0),
	));
}
