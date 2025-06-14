use bevy::prelude::*;
use crate::state::{ GameState, GameStateData, GameStateEvent };
use std::fs;

const LAYOUT_FILEPATH: &str = "./assets/stage_layouts/stage_";
const TILE_SIZE: f32 = 0.84;

pub struct StagePlugin;

impl Plugin for StagePlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, init_stage);
		app.add_systems(Update, (read_gamestate_events, update_stage).chain());
	}
}

#[derive(Bundle)]
struct StageBundle {
	stage: Stage,
	camera: Camera3d,
	transform: Transform,
	spotlight: PointLight,
}

#[derive(Component)]
struct Stage {
	current_stage_id: u32,
	stage_layout: Vec<String>,
	stage_setting_index: u32,
	camera_translation: Vec3,
}

fn init_stage(
	mut commands: Commands,
) {
	commands.spawn(StageBundle{
		stage: Stage::new(),
		camera: Camera3d::default(),
		transform: Transform::from_xyz(0.0, 12.0,0.0).looking_at(Vec3::ZERO, -Vec3::Z),
		spotlight: PointLight { shadows_enabled: true, ..default() },
	});
}

fn read_gamestate_events(
	commands: Commands,
	meshes: ResMut<Assets<Mesh>>,
	materials: ResMut<Assets<StandardMaterial>>,
	mut gamestate_events: EventReader<GameStateEvent>,
	mut query: Query<&mut Stage>
) {
	let mut event_received = false;
	let mut event_data: &GameStateData = &GameStateData::Init;
	
	for e in gamestate_events.read() {
		event_received = true;
		event_data = &e.data;
		break;
	}

	for mut stage in &mut query {
		if event_received {
			match event_data {
				GameStateData::Init => {},
				GameStateData::Setup (setup_data) => {
					stage.load_stage_layout(setup_data.stage_id);
					stage.calculate_camera_translation();
					stage.set_stage(
						commands,
						meshes,
						materials
					);
					break;
				},
				GameStateData::Start => {
					
				},
				GameStateData::Play => {
					
				},
				GameStateData::Win => {
					
				},
				GameStateData::Death => {
					
				},
			}
		}
	}
}

fn update_stage(
	game_state: Res<GameState>,
	time: Res<Time>,
	commands: Commands,
	mut query: Query<(&mut Stage, &mut Camera, &mut PointLight, &mut Transform)>
) {
	for (stage, camera, mut light, mut transform) in query {
		// animate if needed
		let current_translation = transform.translation;
		let almost_equal = current_translation.abs_diff_eq(stage.camera_translation, 0.001);
		if !almost_equal {
			transform.translation = current_translation.lerp(stage.camera_translation, time.delta_secs());
			//dbg!(transform.translation);
		}
		let lookat = Vec3::new(stage.camera_translation.x, 0.0, stage.camera_translation.z);
		transform.look_at(lookat, -Vec3::Z);


	}
}

impl Stage {
	fn new() -> Self {
		Self { 
			current_stage_id: 0, // would be better to init to -1 but u32 for now
			stage_layout: vec![],
			stage_setting_index: 0,
			camera_translation: Vec3::new(0.0, 0.0, 0.0),
		}
	}

	fn load_stage_layout(&mut self, stage_id:u32) {
		self.current_stage_id = stage_id;

		let path = format!("{}{}.txt", LAYOUT_FILEPATH, stage_id);
		let layout = fs::read_to_string(path).expect("level layout {stage_id} not found!");
		
		println!("stage: loaded layout {}:\n{}", stage_id, layout);
		
		self.stage_layout = vec![];
		for line in layout.lines() {
			self.stage_layout.push(String::from(line));
		}
		
		// TODO: validate layout
	}

	fn calculate_camera_translation(&mut self) {
		if self.stage_layout.len() == 0 { self.camera_translation = Vec3::ZERO; }
		
		let mut x: f32 = 0.0;
		let mut z: f32 = 0.0;
		let mut line_length_read = false;
		
		// roundabout way of not doing unsafe casting
		for line in &self.stage_layout {
			z += 1.0;
			if !line_length_read {
				for _c in line.chars() {
					x += 1.0;
				}
				line_length_read = true;
			}
		}

		println!("map height: {z} map width: {x}");

		let y = z + x * 0.5;
		z = z / 2.0 - 0.5;
		x = x / 2.0 - 0.5;

		self.camera_translation = Vec3::new(x, y, z);

		println!("... calculated camera translation");
		dbg!(self.camera_translation);
	}

	fn set_stage(&mut self,
		mut commands: Commands, 
		mut meshes: ResMut<Assets<Mesh>>,
		mut materials: ResMut<Assets<StandardMaterial>>
	) {
		println!("stage: Setting stage {}", self.current_stage_id);
		
		let mut yf: f32 = -1.0;
	
		// we can follow the direction of the lines in a textfile (starting from top),
		// because 3d z+ (treated as 2d y+ in top-down) is towards the player.
		for y in 0..self.stage_layout.len() {
			let line = &self.stage_layout[y];
			println!("...building layout line {}: {}...", y, line);

			yf += 1.0;
			let mut xf = -1.0;

			for c in line.chars() {
				xf += 1.0;

				if c == '_' {
					continue;
				}
				if c == '#' {
					commands.spawn((
						Mesh3d(meshes.add(Cuboid::new(TILE_SIZE, TILE_SIZE, TILE_SIZE))),
						MeshMaterial3d(materials.add(Color::srgb_u8(220, 120, 240))),
						Transform::from_xyz(xf, 0.5, yf), // coordinate swizzle xyz to xzy - top down view
					));
				}
			}
		}
	}
}
