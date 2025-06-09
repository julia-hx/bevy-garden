use bevy::prelude::*;
use crate::state::{ GameStateData, GameStateEvent };
use std::fs;

const DEFAULT_CAMERA_DISTANCE: f32 = 24.0;
const LAYOUT_FILEPATH: &str = "./assets/stage_layouts/stage_";
const TILE_SIZE: f32 = 0.84;

pub struct StagePlugin;

impl Plugin for StagePlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, init_stage);
		app.add_systems(Update, update_stage);
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
	gamestate: GameStateData,
	camera_distance: f32,
}

fn init_stage(
	mut commands: Commands,
) {
	commands.spawn(StageBundle{
		stage: Stage::new(),
		camera: Camera3d::default(),
		transform: Transform::from_xyz(0.0, DEFAULT_CAMERA_DISTANCE, 0.0).looking_at(Vec3::ZERO, -Vec3::Z),
		spotlight: PointLight { shadows_enabled: true, ..default() },
	});
}

fn update_stage(
	commands: Commands,
	meshes: ResMut<Assets<Mesh>>,
	materials: ResMut<Assets<StandardMaterial>>,
	mut gamestate_events: EventReader<GameStateEvent>,
	mut query: Query<(&mut Stage, &mut Camera, &mut PointLight)>
) {
	let mut event_received = false;
	let mut event_data: &GameStateData = &GameStateData::Init;
	
	for e in gamestate_events.read() {
		event_received = true;
		event_data = &e.data;
		break;
	}

	for (mut stage, mut _camera, mut _light) in &mut query {
		if event_received {
			stage.gamestate = event_data.clone(); // TODO: not clone this?
			match stage.gamestate {
				GameStateData::Init => {},
				GameStateData::Setup (setup_data) => {
					stage.load_stage_layout(setup_data.stage_id);
					
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

impl Stage {
	fn new() -> Self {
		Self { 
			current_stage_id: 0, // would be better to init to -1 but u32 for now
			stage_layout: vec![],
			stage_setting_index: 0,
			gamestate: GameStateData::Init,
			camera_distance: DEFAULT_CAMERA_DISTANCE,
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

				// println!("...{} {} {}...", yf, xf, c);

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
