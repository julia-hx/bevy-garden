use bevy::prelude::*;
use crate::state::{ GameState, GameStateData, GameStateEvent };
use std::fs;

const LAYOUT_FILEPATH: &str = "./assets/stage_layouts/stage_";
const TILE_SIZE: f32 = 0.94;
const DEFAULT_SPOTLIGHT_INTENSITY: f32 = 8_000_000.0;
const DEFAULT_STAGE_SETTING_INTERVAL: f32 = 0.5;

pub struct StagePlugin;

impl Plugin for StagePlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, init_stage);
		app.add_systems(Update, (read_gamestate_events, update_stage, update_spotlight).chain());
	}
}

#[derive(Bundle)]
struct StageBundle {
	stage: Stage,
	camera: Camera3d,
	transform: Transform, // acts as camera transform
}

#[derive(Bundle)]
struct SpotlightBundle {
	light: PointLight,
	transform: Transform,
	data: SpotlightData,
}

#[derive(Component)]
struct Stage {
	id: u32,
	layout: Vec<String>,
	width: usize,
	height: usize,
	stage_setting_data: StageSettingData,
	camera_translation: Vec3,
	colors: StageColors,
}

#[derive(Debug, Copy, Clone)]
struct StageColors {
	tiles_a: Color,
	tiles_b: Color,
	tiles_c: Color,
	clear_color: Color,
	snacks: Color,
	ui: Color
}

impl StageColors {
	fn new() -> Self {
		Self {
			tiles_a: Color::srgb_u8(120, 120, 120),
			tiles_b: Color::srgb_u8(60, 60, 60),
			tiles_c: Color::srgb_u8(20, 20, 20),
			clear_color: Color::srgb(0.1, 0.1, 0.12),
			snacks: Color::srgb_u8(220, 220, 60),
			ui: Color::WHITE,
		}
	}
}

#[derive(Debug, Clone)]
struct StageSettingData {
	interval: f32,
	current_line: String,
	progress_x: usize,
	progress_y: usize,
	x: f32,
	y: f32,
	tile_placed_time: f32,
	in_progress: bool,
}

impl StageSettingData {
	fn new() -> Self {
		Self {
			interval: DEFAULT_STAGE_SETTING_INTERVAL,
			current_line: String::from("_"),
			progress_x: 0,
			progress_y: 0,
			x: 0.0,
			y: 0.0,
			tile_placed_time: 0.0,
			in_progress: false,
		}
	}
}

#[derive(Component)]
struct SpotlightData {
	translation: Vec3,
	intensity: f32,
}

impl SpotlightData {
	fn new() -> Self {
		Self {
			translation: Vec3::new(0.0, 10.0, 0.0),
			intensity: DEFAULT_SPOTLIGHT_INTENSITY,
		}
	}
}

fn init_stage(
	mut commands: Commands,
) {
	commands.spawn(StageBundle{
		stage: Stage::new(),
		camera: Camera3d::default(),
		transform: Transform::from_xyz(0.0, 12.0,0.0).looking_at(Vec3::ZERO, -Vec3::Z),
	});

	commands.spawn(SpotlightBundle {
		transform: Transform::from_xyz(0.0, 10.0,0.0),
		light: PointLight { shadows_enabled: true, intensity: 10_000_000.0, ..default() },
		data: SpotlightData::new(),
	});
}

fn read_gamestate_events(
	
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
					stage.load_layout(setup_data.stage_id);
					stage.calculate_camera_translation();
					stage.stage_setting_data = StageSettingData::new();
					stage.stage_setting_data.in_progress = true;
					stage.stage_setting_data.current_line = stage.layout[0].clone();
					println!("stage: Setting stage {}", stage.id);
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
	mut game_state: ResMut<GameState>,
	time: Res<Time>,
	commands: Commands,
	meshes: ResMut<Assets<Mesh>>,
	materials: ResMut<Assets<StandardMaterial>>,
	mut clear_color: ResMut<ClearColor>,
	query: Query<(&mut Stage, &mut Transform)>
) {
	for (mut stage, mut transform) in query {
		match &mut game_state.data {
			GameStateData::Setup(setup_data) => {
				// animate camera
				let current_translation = transform.translation;
				let almost_equal = current_translation.abs_diff_eq(stage.camera_translation, 0.001);
				if !almost_equal {
					transform.translation = current_translation.lerp(stage.camera_translation, time.delta_secs());
				} else {
					transform.translation = stage.camera_translation;
				}
				let lookat = Vec3::new(stage.camera_translation.x, 0.0, stage.camera_translation.z);
				transform.look_at(lookat, -Vec3::Z);	
				// animate clear color
				let cc = clear_color.0.mix(&stage.colors.clear_color, time.delta_secs());
				clear_color.0 = cc;
				// tick stage setting
				stage.update_set_stage(commands, meshes, materials, time.elapsed_secs());

				if !stage.stage_setting_data.in_progress {
					setup_data.setup_done = true; // could be a fancy event but this works as well!
				}
				
				return;
			}
			GameStateData::Play => {
				// snap into place if not already
				transform.translation = stage.camera_translation;
				transform.look_at(Vec3::new(stage.camera_translation.x, 0.0, stage.camera_translation.z), -Vec3::Z);
				clear_color.0 = stage.colors.clear_color;
			}
			_=> { return; }
		}
	}
}

fn update_spotlight(
	game_state: Res<GameState>,
	time: Res<Time>,
	query: Query<(&mut PointLight, &mut Transform, &mut SpotlightData)>
) {
	for (mut point_light, mut transform, mut data) in query {
		match game_state.data {
			GameStateData::Setup(setup_data) => {
				data.translation = setup_data.spotlight_translation;
				data.intensity = DEFAULT_SPOTLIGHT_INTENSITY * setup_data.spotlight_intensity_multiplier;
			}
			_ => {}
		}

		let current_translation = transform.translation;
		let almost_equal = current_translation.abs_diff_eq(data.translation, 0.001);
		if !almost_equal {
			transform.translation = current_translation.lerp(data.translation, time.delta_secs());
		} else {
			transform.translation = data.translation;
		}

		let current_intensity = point_light.intensity;
		let animated_intensity = current_intensity.lerp(data.intensity, time.delta_secs());
		point_light.intensity = animated_intensity;		
	}
}

impl Stage {
	fn new() -> Self {
		Self { 
			id: 0, // would be better to init to -1 but u32 for now
			layout: vec![],
			stage_setting_data: StageSettingData::new(),
			width: 0,
			height: 0,
			camera_translation: Vec3::new(0.0, 0.0, 0.0),
			colors: StageColors::new(),
		}
	}

	fn load_layout(&mut self, stage_id:u32) {
		self.id = stage_id;

		println!("stage: attempting to load layout for id {}", stage_id);

		let path = format!("{}{}.txt", LAYOUT_FILEPATH, stage_id);
		let layout = fs::read_to_string(path).expect("level layout {stage_id} not found!");
		
		println!("stage: loaded layout {}:\n{}", stage_id, layout);
		// TODO: validate layout
		
		self.layout = vec![];
		for line in layout.lines() {
			self.layout.push(String::from(line));
		}
		
		self.height = self.layout.len();
		self.width = self.layout[0].len();

		println!("stage height: {} width: {}", self.height, self.width);
	}

	fn calculate_camera_translation(&mut self) {
		if self.layout.len() == 0 { self.camera_translation = Vec3::ZERO; }
		
		let mut x: f32 = 0.0;
		let mut z: f32 = 0.0;
		let mut line_length_set = false;
		
		// roundabout way of not doing unsafe casting
		for _i in 0..self.height {
			z += 1.0;
			if !line_length_set {
				for _j in 0..self.width {
					x += 1.0;
				}
				line_length_set = true;
			}
		}

		let y = (z + x * 0.5) * 1.4;
		z = z / 2.0 - 0.5;
		x = x / 2.0 - x / 10.0;

		self.camera_translation = Vec3::new(x, y, z);

		println!("... calculated camera translation");
		dbg!(self.camera_translation);
	}

	fn update_set_stage(&mut self,
		mut commands: Commands,
		mut meshes: ResMut<Assets<Mesh>>,
		mut materials: ResMut<Assets<StandardMaterial>>,
		time: f32,
	) {
		
		let data = &mut self.stage_setting_data;
		if !data.in_progress { return; }
		if time < data.tile_placed_time + data.interval { return; }
		
		// set tile at current x and y
		let c = data.current_line.chars()
			.nth(data.progress_x)
			.unwrap_or('_');

		match c {
			'A' => {
				commands.spawn((
					Mesh3d(meshes.add(Cuboid::new(TILE_SIZE, TILE_SIZE, TILE_SIZE))),
					MeshMaterial3d(materials.add(self.colors.tiles_a)),
					Transform::from_xyz(data.x, 0.5, data.y), // coordinate swizzle xyz to xzy - top down view
				));
			}
			'B' => {
				commands.spawn((
					Mesh3d(meshes.add(Cuboid::new(TILE_SIZE, TILE_SIZE, TILE_SIZE))),
					MeshMaterial3d(materials.add(self.colors.tiles_b)),
					Transform::from_xyz(data.x, 0.5, data.y),
				));
			}
			'C' => {
				commands.spawn((
					Mesh3d(meshes.add(Cuboid::new(TILE_SIZE, TILE_SIZE, TILE_SIZE))),
					MeshMaterial3d(materials.add(self.colors.tiles_c)),
					Transform::from_xyz(data.x, 0.5, data.y),
				));
			}
			_ => {}
		}

		// tick x and y
		if data.progress_x < self.width - 1 { // move through line
			data.progress_x += 1;
			data.x += 1.0;
		} else if data.progress_y < self.height - 1 { // get next line
			data.progress_x = 0;
			data.x = 0.0;
			data.progress_y += 1;
			data.y += 1.0;
			data.current_line = self.layout[data.progress_y].clone();
		} else { // done!
			data.in_progress = false; 
		}

		if data.interval > 0.05 {
			data.interval = data.interval * 0.86;
		} else {
			data.interval = 0.001; // tick more or less every frame for the rest
		}
		data.tile_placed_time = time;

		/*
		let mut yf: f32 = -1.0;

		// we can follow the direction of the lines in a textfile (starting from top),
		// because 3d z+ (treated as 2d y+ in top-down) is towards the player.
		for y in 0..self.layout.len() {
			let line = &self.layout[y];
			println!("...building layout line {}: {}...", y, line);

			yf += 1.0;
			let mut xf = -1.0;

			for c in line.chars() {
				match c {
					'_' => { continue; }
					'A' => {
						commands.spawn((
							Mesh3d(meshes.add(Cuboid::new(TILE_SIZE, TILE_SIZE, TILE_SIZE))),
							MeshMaterial3d(materials.add(self.colors.tiles_a)),
							Transform::from_xyz(xf, 0.5, yf), // coordinate swizzle xyz to xzy - top down view
						));
					}
					'B' => {
						commands.spawn((
							Mesh3d(meshes.add(Cuboid::new(TILE_SIZE, TILE_SIZE, TILE_SIZE))),
							MeshMaterial3d(materials.add(self.colors.tiles_b)),
							Transform::from_xyz(xf, 0.5, yf), // coordinate swizzle xyz to xzy - top down view
						));
					}
					'C' => {
						commands.spawn((
							Mesh3d(meshes.add(Cuboid::new(TILE_SIZE, TILE_SIZE, TILE_SIZE))),
							MeshMaterial3d(materials.add(self.colors.tiles_c)),
							Transform::from_xyz(xf, 0.5, yf), // coordinate swizzle xyz to xzy - top down view
						));
					}
					_ => {}
				}
				xf += 1.0;
			}
		}

		println!("...stage setting done!");
		self.stage_setting_data.in_progress = false;
		*/
	}
}
