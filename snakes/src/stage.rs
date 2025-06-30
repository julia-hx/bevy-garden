use bevy::prelude::*;
use crate::state::{ GameState, GameStateData, GameStateEvent, PlayData, WinData };
use std::fs;
use rand::prelude::*;

const LAYOUT_FILEPATH: &str = "./assets/stage_layouts/stage_";
const TILE_SIZE: f32 = 0.94;
const DEFAULT_SPOTLIGHT_INTENSITY: f32 = 8_000_000.0;
const DEFAULT_STAGE_SETTING_INTERVAL: f32 = 0.5;
const GLITTER_INTERVAL: f32 = 0.03;

// stage plugin: set stage from textfile data,
// evaluate snake movements against walkable masks and snack location.

pub struct StagePlugin;

impl Plugin for StagePlugin {
	fn build(&self, app: &mut App) {
		app.add_event::<StageEvent>();
		app.add_systems(Startup, init_stage);
		app.add_systems(Update, (
			read_gamestate_events,
			update_stage,
			update_tiles,
			update_spotlight,
		).chain());
	}
}

#[derive(Event)]
pub struct StageEvent {
	pub data: StageEventData,
}

#[derive(Clone)]
pub enum StageEventData {
	Empty,
	SetSnakeSpawnPoint(SnakeSpawnPointData),
	SpawnSnack(StageCoordinate), // coordinate
	ClearSnack,
	SnackEaten(u32), // snake id
	SnakeFalling(u32), // snake id
}

#[derive(Clone, Copy)]
pub struct SnakeSpawnPointData {
	pub snake_id: u32,
	pub spawn_point: StageCoordinate,
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
	walkable: StageWalkableMask,
	snack_coordinate: StageCoordinate,
	snack_spawntime: f32,
}

#[derive(Component)]
struct Tile {

}

impl Tile {
	fn new() -> Self {
		Tile {}
	}
}

#[derive(Debug, Copy, Clone)]
struct StageColors {
	tiles_a: Color,
	tiles_b: Color,
	tiles_c: Color,
	clear_color: Color,
}

impl StageColors {
	fn new() -> Self {
		Self {
			tiles_a: Color::srgb_u8(120, 120, 120),
			tiles_b: Color::srgb_u8(60, 60, 60),
			tiles_c: Color::srgb_u8(20, 20, 20),
			clear_color: Color::srgb(0.1, 0.1, 0.12),
		}
	}
}

#[derive(Debug, Clone)]
struct StageSettingData {
	interval: f32,
	current_line: String,
	x: usize,
	y: usize,
	tile_placed_time: f32,
	in_progress: bool,
}

impl StageSettingData {
	fn new() -> Self {
		Self {
			interval: DEFAULT_STAGE_SETTING_INTERVAL,
			current_line: String::from("_"),
			x: 0,
			y: 0,
			tile_placed_time: 0.0,
			in_progress: false,
		}
	}
}

#[derive(Copy, Clone, Debug)]
pub struct StageCoordinate {
	pub x: i32,
	pub y: i32,
}

impl StageCoordinate {
	pub const fn new(x: i32, y: i32) -> Self { Self {x, y} }
	
	pub fn equals(&self, other: &StageCoordinate) -> bool {
		self.x == other.x && self.y == other.y
	}
}

#[derive(Debug, Clone)]
pub struct StageWalkableRow {
	pub tiles: Vec<bool>,
}

#[derive(Debug, Clone)]
pub struct StageWalkableMask {
	pub rows: Vec<StageWalkableRow>,
}

impl StageWalkableMask {
	pub fn new(width: usize, height: usize) -> Self {		
		let mut s: StageWalkableMask = StageWalkableMask { rows: vec![] };
		s.init(width, height);
		s
	}

	pub fn init(&mut self, width: usize, height: usize) {
		self.rows.clear();
		// create a register of walkable true/false data the size of the map layout.
		for y in 0..height {
			self.rows.push(StageWalkableRow { tiles:vec![] });
			for _x in 0..width {
				// walkable by default.
				self.rows[y].tiles.push(true);
			}  
		}
	}

	pub fn set(&mut self, coordinate: &StageCoordinate, value: bool) {
		if !self.contains(coordinate) { return; }
		self.rows[coordinate.y as usize].tiles[coordinate.x as usize] = value;
	}

	pub fn get(&mut self, coordinate: &StageCoordinate) -> bool {
		if !self.contains(coordinate) { return false; }
		self.rows[coordinate.y as usize].tiles[coordinate.x as usize]
	}

	pub fn contains(&mut self, coordinate: &StageCoordinate) -> bool {
		if coordinate.y < 0 || coordinate.y >= self.rows.len() as i32 { return false; }
		if coordinate.x < 0 || coordinate.x >= self.rows[coordinate.y as usize].tiles.len() as i32 { return false; }
		true
	}

	pub fn print(&self) {
		for row in &self.rows {
			let mut print_row: String = String::new();
			for tile in &row.tiles {
				let c = if *tile { '1' } else { '0' };
				print_row.push(c);
			}
			println!("{print_row}");
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
	mut event_writer: EventWriter<StageEvent>,
	mut query: Query<&mut Stage>
) {
	let event_data: &GameStateData;

	if let Some(e) = gamestate_events.read().next() {
		event_data = &e.data;
	} else { return; }

	for mut stage in &mut query {
		match event_data {
			GameStateData::Init => {},
			GameStateData::Setup (setup_data) => {
				stage.load_layout(setup_data.stage_id);
				stage.calculate_height_and_width_from_layout();
				stage.calculate_camera_translation();
				let width = stage.width;
				let height = stage.height;
				stage.walkable.init(width, height);
				stage.stage_setting_data = StageSettingData::new();
				stage.stage_setting_data.in_progress = true;
				stage.stage_setting_data.current_line = stage.layout[0].clone();
				
				println!("stage: setting stage {}", stage.id);
				break;
			}
			GameStateData::Start => {
				println!("stage walkable mask:");
				stage.walkable.print();
				break;
			}
			GameStateData::Play (_play_data)=> {
				
			}
			GameStateData::Win ( _win_data)=> {
				event_writer.write(StageEvent { data: StageEventData::ClearSnack });
			}
			GameStateData::Death => {
				
			}
			GameStateData::Reset(_counter) => {
				event_writer.write(StageEvent { data: StageEventData::ClearSnack });
			}
		}
	}
}

fn update_stage(
	mut event_writer: EventWriter<StageEvent>,
	mut game_state: ResMut<GameState>,
	time: Res<Time>,
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
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
				if setup_data.fast_forward {
					for _i in 0..stage.get_tiles_left() {
						stage.update_set_stage(&mut event_writer, &mut commands, &mut meshes, &mut materials, time.elapsed_secs(), setup_data.fast_forward);
					}
				} else {
					stage.update_set_stage(&mut event_writer, &mut commands, &mut meshes, &mut materials, time.elapsed_secs(), setup_data.fast_forward);
				}

				if !stage.stage_setting_data.in_progress {
					setup_data.setup_done = true; // don't cross the event streams, maybe?
					// accessing state data directly causes less bugs than writing and reading events in opposite direction between stage / state / snake.
					// state -> stage & snake / stage -> snake works.
					// want to try a single event stream with many different types in some other garden project.
					//
					// ... also would like to make sure this is actually the thing causing problems.
				}
				
				return;
			}
			GameStateData::Start => {
				
				// snap into place if not already
				transform.translation = stage.camera_translation;
				transform.look_at(Vec3::new(stage.camera_translation.x, 0.0, stage.camera_translation.z), -Vec3::Z);
				clear_color.0 = stage.colors.clear_color;

				game_state.stage_width = stage.width;
				game_state.stage_height = stage.height;

				// we're spamming data here... but this state doesn't do much else except waiting for player to press play.

				return;
			}
			GameStateData::Play (play_data) => {				
				let mut snake_data: Vec<(u32, &StageCoordinate, bool)> = vec![];
				if play_data.snake1_data.active && !play_data.snake1_data.falling { snake_data.push((1, &play_data.snake1_data.coordinate, play_data.snake1_data.evaluate_move)) };
				if play_data.snake2_data.active && !play_data.snake2_data.falling { snake_data.push((2, &play_data.snake2_data.coordinate, play_data.snake1_data.evaluate_move)) };
				if play_data.snake3_data.active && !play_data.snake3_data.falling { snake_data.push((3, &play_data.snake3_data.coordinate, play_data.snake1_data.evaluate_move)) };

				for (snake_id, snake_coordinate, evaluate_move) in snake_data {
					// nothing to evaluate if snake hasn't moved
					if !evaluate_move { return; }
					
					// falling snakes?
					if !stage.walkable.get(snake_coordinate) {
						// check if already falling
						match snake_id {
							1 => { if play_data.snake1_data.falling { continue; } }
							2 => { if play_data.snake2_data.falling { continue; } }
							3 => { if play_data.snake3_data.falling { continue; } }
						    _=> {}
						}
						event_writer.write(StageEvent { data: StageEventData::SnakeFalling(snake_id) });
						continue;
					}
					// snack eaten?
					if snake_coordinate.equals(&stage.snack_coordinate) {
						// increase score and movement speed, flag that it's time to update ui
						play_data.score += 1;
						play_data.move_speed += play_data.move_speed_increment;
						play_data.someone_had_a_snack = true;
						println!("... score is now {} of {}", play_data.score, play_data.goal);
						println!("... move speed is now {}", play_data.move_speed);
						// update ui
						if play_data.score >= play_data.goal { 
							event_writer.write(StageEvent { data: StageEventData::ClearSnack });
							continue;
						}
						event_writer.write(StageEvent { data: StageEventData::SnackEaten(snake_id) });
						stage.snack_coordinate = stage.get_next_snack_coordinate(play_data);
						stage.snack_spawntime = time.elapsed_secs();
						event_writer.write(StageEvent { data: StageEventData::SpawnSnack(stage.snack_coordinate) });
						continue;
					}

					match snake_id {
						1 => { play_data.snake1_data.evaluate_move = false; }
						2 => { play_data.snake2_data.evaluate_move = false; }
						3 => { play_data.snake3_data.evaluate_move = false; }
						_=> ()
					}
				}
				return;
			}
			GameStateData::Win (win_data) => {
				if time.elapsed_secs() >= stage.snack_spawntime + GLITTER_INTERVAL {
					stage.snack_coordinate = stage.get_next_snack_coordinate(&win_data.play_data);
					stage.snack_spawntime = time.elapsed_secs();
					event_writer.write(StageEvent { data: StageEventData::SpawnSnack(stage.snack_coordinate) });
				}
			}
			_=> {}
		}
	}
}

fn update_tiles(
	mut game_state: ResMut<GameState>,
	time: Res<Time>,
	mut commands: Commands,
	query: Query<(Entity, &Tile, &mut Transform)>
) {
	match &mut game_state.data {
		GameStateData::Reset(_counter) => {
			for (entity, _tile, _transform) in query {
				commands.entity(entity).despawn();
			}
		}
		_=> {}
	}
}

fn update_spotlight(
	game_state: Res<GameState>,
	time: Res<Time>,
	query: Query<(&mut PointLight, &mut Transform, &mut SpotlightData)>
) {
	for (mut point_light, mut transform, mut data) in query {
		if let GameStateData::Setup(setup_data) = game_state.data {
			data.translation = setup_data.spotlight_translation;
			data.intensity = DEFAULT_SPOTLIGHT_INTENSITY * setup_data.spotlight_intensity_multiplier;
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
			id: 0,
			layout: vec![],
			stage_setting_data: StageSettingData::new(),
			width: 0,
			height: 0,
			camera_translation: Vec3::new(0.0, 0.0, 0.0),
			colors: StageColors::new(),
			walkable: StageWalkableMask::new(0, 0),
			snack_coordinate: StageCoordinate::new(0, 0),
			snack_spawntime: 0.0
		}
	}

	fn load_layout(&mut self, stage_id:u32) {
		self.id = stage_id;

		println!("stage: attempting to load layout for id {}", stage_id);

		let path = format!("{}{}.txt", LAYOUT_FILEPATH, stage_id);
		let layout = fs::read_to_string(path).expect("level layout {stage_id} not found!");
		
		println!("stage loaded layout {}:\n{}", stage_id, layout);
		// TODO: validate layout
		
		self.layout = vec![];
		for line in layout.lines() {
			self.layout.push(String::from(line));
		}
		
		self.height = self.layout.len();
		self.width = self.layout[0].len();
	}

	fn calculate_height_and_width_from_layout(&mut self) {
		if self.layout.is_empty() {
			self.height = 0;
			self.width = 0;
			return;
		}
		
		self.height = self.layout.len();
		self.width = self.layout[0].len();

		println!("stage height: {} width: {}", self.height, self.width);
	} 

	fn calculate_camera_translation(&mut self) {
		if self.layout.is_empty() { self.camera_translation = Vec3::ZERO; }
		
		let mut x = self.width as f32;
		let mut z = self.height as f32;
		
		x = if self.width % 2 == 0 { x / 2.0 } else { x / 2.0 - 0.5 };
		z = if self.height % 2 == 0 { z / 2.0 } else { z / 2.0 - 0.5 };

		let y = (z + x) * 1.68;

		self.camera_translation = Vec3::new(x, y, z);

		println!("... calculated camera translation");
		dbg!(self.camera_translation);
	}

	// this looks a lot like it could be a system - 
	// having access to self does make a lot of internal data 
	// access much easier than handling refs and borrowing.
	fn update_set_stage(&mut self,
		event_writer: &mut EventWriter<StageEvent>,
		commands: &mut Commands,
		meshes: &mut ResMut<Assets<Mesh>>,
		materials: &mut ResMut<Assets<StandardMaterial>>,
		time: f32,
		fast_forward: bool,
	) {
		let data = &mut self.stage_setting_data;
		if !data.in_progress { return; }
		if !fast_forward && time < data.tile_placed_time + data.interval { return; }
		
		// set tile at current x and y
		let c = data.current_line.chars()
			.nth(data.x)
			.unwrap_or('_');

		match c {
			'A' => {
				commands.spawn((
					Tile::new(),
					Mesh3d(meshes.add(Cuboid::new(TILE_SIZE, TILE_SIZE, TILE_SIZE))),
					MeshMaterial3d(materials.add(self.colors.tiles_a)),
					Transform::from_xyz(data.x as f32, 0.5, data.y as f32), // coordinate swizzle xyz to xzy - top down view
				));
			}
			'B' => {
				commands.spawn((
					Tile::new(),
					Mesh3d(meshes.add(Cuboid::new(TILE_SIZE, TILE_SIZE, TILE_SIZE))),
					MeshMaterial3d(materials.add(self.colors.tiles_b)),
					Transform::from_xyz(data.x as f32, 0.5, data.y as f32),
				));
			}
			'C' => {
				commands.spawn((
					Tile::new(),
					Mesh3d(meshes.add(Cuboid::new(TILE_SIZE, TILE_SIZE, TILE_SIZE))),
					MeshMaterial3d(materials.add(self.colors.tiles_c)),
					Transform::from_xyz(data.x as f32, 0.5, data.y as f32),
				));
			}
			'1' => {
				commands.spawn((
					Tile::new(),
					Mesh3d(meshes.add(Cuboid::new(TILE_SIZE, TILE_SIZE, TILE_SIZE))),
					MeshMaterial3d(materials.add(self.colors.tiles_a)),
					Transform::from_xyz(data.x as f32, 0.5, data.y as f32),
				));
				let snake_spawn_point_data = SnakeSpawnPointData{ snake_id: 1, spawn_point: StageCoordinate::new(data.x as i32, data.y as i32) };
				event_writer.write(StageEvent { data: StageEventData::SetSnakeSpawnPoint(snake_spawn_point_data) });
			}
			'2' => {
				commands.spawn((
					Tile::new(),
					Mesh3d(meshes.add(Cuboid::new(TILE_SIZE, TILE_SIZE, TILE_SIZE))),
					MeshMaterial3d(materials.add(self.colors.tiles_a)),
					Transform::from_xyz(data.x as f32, 0.5, data.y as f32),
				));
				let snake_spawn_point_data = SnakeSpawnPointData{ snake_id: 2, spawn_point: StageCoordinate::new(data.x as i32, data.y as i32) };
				event_writer.write(StageEvent { data: StageEventData::SetSnakeSpawnPoint(snake_spawn_point_data) });
			}
			'3' => {
				commands.spawn((
					Tile::new(),
					Mesh3d(meshes.add(Cuboid::new(TILE_SIZE, TILE_SIZE, TILE_SIZE))),
					MeshMaterial3d(materials.add(self.colors.tiles_a)),
					Transform::from_xyz(data.x as f32, 0.5, data.y as f32),
				));
				let snake_spawn_point_data = SnakeSpawnPointData{ snake_id: 3, spawn_point: StageCoordinate::new(data.x as i32, data.y as i32) };
				event_writer.write(StageEvent { data: StageEventData::SetSnakeSpawnPoint(snake_spawn_point_data) });
			}
			'*' => {
				commands.spawn((
					Tile::new(),
					Mesh3d(meshes.add(Cuboid::new(TILE_SIZE, TILE_SIZE, TILE_SIZE))),
					MeshMaterial3d(materials.add(self.colors.tiles_a)),
					Transform::from_xyz(data.x as f32, 0.5, data.y as f32),
				));
				event_writer.write(StageEvent { data: StageEventData::SpawnSnack(StageCoordinate::new(data.x as i32, data.y as i32)) });
				self.snack_coordinate.x = data.x as i32;
				self.snack_coordinate.y = data.y as i32;
			}
			_ => {
				// if no tile was placed, mark the coordinate as non-walkable:
				self.walkable.rows[data.y].tiles[data.x] = false;
			}
		}

		// tick x and y
		if data.x < self.width - 1 { // move through line
			data.x += 1;
		} else if data.y < self.height - 1 { // get next line
			data.x = 0;
			data.y += 1;
			data.current_line = self.layout[data.y].clone();
		} else { // done!
			data.in_progress = false; 
		}

		if data.interval > 0.01 {
			data.interval *= 0.86;
		} else {
			data.interval = 0.001; // tick more or less every frame for the rest
		}
		data.tile_placed_time = time;
	}

	fn get_tiles_left(&mut self) -> usize {
		self.height * self.width - ( (self.stage_setting_data.x + 1) + (self.stage_setting_data.y + 1) )
	}

	fn get_next_snack_coordinate(&mut self, play_data: &PlayData) -> StageCoordinate {
		let mut rng = rand::rng();
		let mut candidates: Vec<StageCoordinate> = vec![];

		for y in 0..self.height {
			if play_data.snakes_walkable_mask.rows.len() <= y { break; }
			
			for x in 0..self.width {
				if play_data.snakes_walkable_mask.rows[y].tiles.len() <= x { break; }
				
				let coordinate = StageCoordinate::new(x as i32, y as i32);
				if play_data.snakes_walkable_mask.rows[y].tiles[x] && self.walkable.get(&coordinate) {
					candidates.push(coordinate);
				}			
			}
		}

		if !candidates.is_empty() {
			let ri = rng.random_range(0..candidates.len());
			candidates[ri]
		}
		else { StageCoordinate::new(0, 0) }
	}
}
