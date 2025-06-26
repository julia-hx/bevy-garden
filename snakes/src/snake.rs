use bevy::{input::keyboard::KeyboardInput, prelude::*};
use bevy::time::common_conditions::on_timer;

use crate::state::{ GameState, GameStateData, GameStateEvent, SnakePlayData };
use crate::stage::{ StageCoordinate, StageEvent, StageEventData };

use std::time::Duration;

const SNAKE_HEAD_SIZE: Vec3 = Vec3::new(1.0, 0.8, 1.0);
const SNAKE_SEGMENT_SIZE: Vec3 = Vec3::new(0.68, 0.6, 0.68);
const SNAKE_Y: f32 = 1.4;
const HIDDEN_COORDINATE: StageCoordinate = StageCoordinate::new(1000, 1000);

const SNAKE_COLOR_1: Color = Color::srgb_u8(220, 100, 220);
const SNAKE_COLOR_2: Color = Color::srgb_u8(80, 220, 220);
const SNAKE_COLOR_3: Color = Color::srgb_u8(120, 220, 120);

const DEBUG_SNAKES_WALKABLE_MASK: bool = false;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, init_snakes);
		app.add_systems(Update,
			(
				read_gamestate_events,
				read_stage_events,
				read_input,
				move_snakes,
				spawn_segments,
				move_segments,
				despawn_segments,
				evaluate_all_falling.run_if(on_timer(Duration::from_secs(2))),
			).chain()
		);
	}
}

#[derive(Component)]
pub struct Snake {
	pub id: u32,
	pub direction: Direction,
	pub last_direction_moved: Direction,
	pub falling: bool,
	pub fall_duration: u32,
	pub segments: u32,
	stage_coordinate: StageCoordinate,
	pub active: bool,
	pub input_received: bool,
	pub had_a_snack: bool,
}

impl Snake {
	fn new(id: u32, active: bool) -> Self {
		Self {
			id,
			direction: Direction::Up,
			last_direction_moved: Direction::None,
			falling: false,
			fall_duration: 0,
			segments: 0,
			stage_coordinate: HIDDEN_COORDINATE,
			active,
			input_received: false,
			had_a_snack: false
		}
	}

	fn set_direction(&mut self, direction: Direction) {
		if is_opposite_direction(&self.last_direction_moved, &direction) {
			println!("-- snake can't turn around on itself!");
			return;
		}
		self.direction = direction;
	}
}

#[derive(Component, Debug)]
pub struct Segment {
	snake_id: u32,
	coordinate: StageCoordinate,
	move_counter: u32, // used to determine when to move each segment
}

impl Segment {
	fn new(snake_id: u32, coordinate: StageCoordinate) -> Self {
		Self { 
			snake_id,
			coordinate,
			move_counter: 0,
		}
	}
}

#[derive(Component)]
pub struct InputMapping {
	pub up: KeyCode,
	pub down: KeyCode,
	pub left: KeyCode,
	pub right: KeyCode,
}

#[derive(Component, Copy, Clone)]
pub enum Direction {
	None,
	Up,
	Down,
	Left,
	Right,
}

impl InputMapping {
	fn new(
		up: KeyCode, 
		down: KeyCode, 
		left: KeyCode, 
		right: KeyCode
	) -> Self {
		Self {
			up, down, left, right
		}
	}
}

fn init_snakes(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	println!("init snakes");

	commands.spawn((
		Snake::new(1, true),
		Direction::Up,
		InputMapping::new(
			KeyCode::ArrowUp,
			KeyCode::ArrowDown,
			KeyCode::ArrowLeft,
			KeyCode::ArrowRight),
		Transform::from_xyz(HIDDEN_COORDINATE.x as f32, 0.0, HIDDEN_COORDINATE.y as f32),
		Mesh3d(meshes.add(Cuboid::new(SNAKE_HEAD_SIZE.x, SNAKE_HEAD_SIZE.y, SNAKE_HEAD_SIZE.z))),
		MeshMaterial3d(materials.add(SNAKE_COLOR_1)),
	));

	commands.spawn((
		Snake::new(2, false),
		Direction::Up,
		InputMapping::new(
			KeyCode::KeyW,
			KeyCode::KeyS,
			KeyCode::KeyA,
			KeyCode::KeyD),
		Transform::from_xyz(HIDDEN_COORDINATE.x as f32, 0.0, HIDDEN_COORDINATE.y as f32),
		Mesh3d(meshes.add(Cuboid::new(SNAKE_HEAD_SIZE.x, SNAKE_HEAD_SIZE.y, SNAKE_HEAD_SIZE.z))),
		MeshMaterial3d(materials.add(SNAKE_COLOR_2)),
	));

	commands.spawn((
		Snake::new(3, false),
		Direction::Up,
		InputMapping::new(
			KeyCode::KeyI,
			KeyCode::KeyK,
			KeyCode::KeyJ,
			KeyCode::KeyL),
		Transform::from_xyz(HIDDEN_COORDINATE.x as f32, 0.0, HIDDEN_COORDINATE.y as f32),
		Mesh3d(meshes.add(Cuboid::new(SNAKE_HEAD_SIZE.x, SNAKE_HEAD_SIZE.y, SNAKE_HEAD_SIZE.z))),
		MeshMaterial3d(materials.add(SNAKE_COLOR_3)),
	));
}

fn read_gamestate_events(
	mut gamestate_events: EventReader<GameStateEvent>,
	mut query: Query<(&mut Snake, &mut Transform)>,
) {
	for e in gamestate_events.read() {
		for (mut snake, mut transform) in &mut query {
			match &e.data {
				GameStateData::Init => {},
				GameStateData::Setup (_setup_data)=> { 
					snake.falling = false;
					snake.fall_duration = 0;
					snake.segments = 0;
					snake.last_direction_moved = Direction::None;
					snake.direction = Direction::Up;
					snake.input_received = false;
					snake.stage_coordinate = HIDDEN_COORDINATE;
				},
				GameStateData::Start => {
					if snake.active {
						transform.translation = Vec3::new(snake.stage_coordinate.x as f32, SNAKE_Y, snake.stage_coordinate.y as f32);
					}
				},
				GameStateData::Play (_play_data) => {
				},
				GameStateData::Win (_win_data) => {},
				GameStateData::Death => {},
				GameStateData::Reset(_counter) => {
					transform.translation = Vec3::new(HIDDEN_COORDINATE.x as f32, SNAKE_Y, HIDDEN_COORDINATE.y as f32);
				}
			}
		}
	}
	
}

fn read_stage_events (
	mut stage_events: EventReader<StageEvent>,
	mut game_state: ResMut<GameState>,
	mut query: Query<&mut Snake>,
) {
	for e in stage_events.read() {
		for mut snake in &mut query {
			match e.data {
				StageEventData::SetSnakeSpawnPoint(spawn_point_data) => {
					if spawn_point_data.snake_id != snake.id { continue; }
					snake.stage_coordinate = spawn_point_data.spawn_point;
				}
				StageEventData::SnackEaten(snake_id) => {
					if let GameStateData::Play(play_data) = &mut game_state.data {
						if snake_id == snake.id {
							println!("snake {} had a lil snack!", snake_id);
							snake.had_a_snack = true;
							play_data.increment_speed();
						}
					}
				}
				StageEventData::SnakeFalling(snake_id) => {
					if let GameStateData::Play(_play_data) = &mut game_state.data {
						if snake_id == snake.id {
							snake.falling = true;
							println!("snake {} is falling!", &snake_id);
						}
					}
				}
				_ => {}
			}
		}
	}
}

fn read_input(
	mut key_events: EventReader<KeyboardInput>,
	mut query: Query<(&mut Snake, &mut InputMapping)>,
) {	
	for e in key_events.read() {
		for (mut snake, input_mapping) in &mut query {
			if e.key_code == input_mapping.up { snake.set_direction(Direction::Up); snake.input_received = true; }
			else if e.key_code == input_mapping.down { snake.set_direction(Direction::Down); snake.input_received = true; }
			else if e.key_code == input_mapping.left { snake.set_direction(Direction::Left); snake.input_received = true; }
			else if e.key_code == input_mapping.right { snake.set_direction(Direction::Right); snake.input_received = true; }
		}
	}
} 

fn move_snakes(
	time: Res<Time>,
	mut game_state: ResMut<GameState>,
	query: Query<(&mut Snake, &mut Transform)>,
) {
	match &mut game_state.data {
		GameStateData::Start => {
			for(mut snake, mut transform) in query {
				if !snake.active && snake.input_received { 
					snake.active = true;
					transform.translation = Vec3::new(snake.stage_coordinate.x as f32, SNAKE_Y, snake.stage_coordinate.y as f32);
				}
			}
		}
		GameStateData::Play (play_data) => {
			let mut snakes_moved = false;
			
			for(mut snake, mut transform) in query {
				let snake_data: &mut SnakePlayData = match snake.id {
					1 => { &mut play_data.snake1_data }
					2 => { &mut play_data.snake2_data }
					3 => { &mut play_data.snake3_data }
					_=> { return; }
				};

				snake_data.active = snake.active;
				snake_data.falling = snake.falling;
				snake_data.fall_duration = snake.fall_duration;
				snake_data.previous_coordinate = snake.stage_coordinate;
				snake_data.segments = snake.segments;

				if !snake.active { continue; }
				if play_data.last_move_time + play_data.move_interval >= time.elapsed_secs() { continue; }

				let next_translation: Vec3;

				if snake.falling {
					snake.fall_duration += 1;
					next_translation = Vec3::new(snake.stage_coordinate.x as f32, SNAKE_Y - snake.fall_duration as f32, snake.stage_coordinate.y as f32);
				}	
				else {
					match snake.direction {
						Direction::Up => { snake.stage_coordinate.y -= 1; }
						Direction::Down => { snake.stage_coordinate.y += 1; }
						Direction::Left => { snake.stage_coordinate.x -= 1; }
						Direction::Right => { snake.stage_coordinate.x += 1; }
						Direction::None => {}
					}
					snake.last_direction_moved = snake.direction;
					next_translation = Vec3::new(snake.stage_coordinate.x as f32, SNAKE_Y, snake.stage_coordinate.y as f32);
				}

				if play_data.snakes_walkable_mask.contains(&snake.stage_coordinate) 
				&& !play_data.snakes_walkable_mask.get(&snake.stage_coordinate) 
				&& !snake.falling {
					// crash!
					println!("woops snake {} crashed!", snake.id);
					play_data.crash = true;
				}

				snake_data.coordinate = snake.stage_coordinate;
				snake_data.refresh_segments = true;
				snake_data.evaluate_move = true;
				
				transform.translation = next_translation;

				play_data.snakes_walkable_mask.set(&snake_data.coordinate, false);
				if snake.segments == 0 { play_data.snakes_walkable_mask.set(&snake_data.previous_coordinate, true); }
				snakes_moved = true;
			}

			if snakes_moved { play_data.last_move_time = time.elapsed_secs(); }

			if DEBUG_SNAKES_WALKABLE_MASK {
				println!(" ");
				play_data.snakes_walkable_mask.print();
			}
		}
		_ => {}
	}
}

fn spawn_segments(
	mut game_state: ResMut<GameState>,
	query: Query<&mut Snake>,
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	if let GameStateData::Play(play_data) = &mut game_state.data {
		for mut snake in query {
			if !snake.had_a_snack { continue; }
			let color: Color = match snake.id {
				1 => { SNAKE_COLOR_1 }
				2 => { SNAKE_COLOR_2 }
				3 => { SNAKE_COLOR_3 }
				_ => { Color::srgb(0.4, 0.4, 0.4) }
			};
			commands.spawn((
				Segment::new(snake.id, snake.stage_coordinate),
				Transform::from_xyz(snake.stage_coordinate.x as f32, SNAKE_Y, snake.stage_coordinate.y as f32),
				Mesh3d(meshes.add(Cuboid::new(SNAKE_SEGMENT_SIZE.x, SNAKE_SEGMENT_SIZE.y, SNAKE_SEGMENT_SIZE.z))),
				MeshMaterial3d(materials.add(color)),
			));
			snake.segments += 1;
			snake.had_a_snack = false;
			// this data lags behind until next time segments are moved
			match snake.id {
				1 => { play_data.snake1_data.had_a_snack = true; }
				2 => { play_data.snake2_data.had_a_snack = true; }
				3 => { play_data.snake3_data.had_a_snack = true; }
				_ => {}
			}
		}
	}
}

fn move_segments(
	mut game_state: ResMut<GameState>,
	mut query: Query<(&mut Segment, &mut Transform)>,
) {	
	if let GameStateData::Play(play_data) = &mut game_state.data {
		for (mut segment, mut transform) in &mut query {
			let snake_data = match segment.snake_id {
				1 => { &play_data.snake1_data }
				2 => { &play_data.snake2_data }
				3 => { &play_data.snake3_data }
				_ => { return; }
			};

			if !snake_data.refresh_segments { return; }
			
			segment.move_counter += 1;

			// not all segments are moved every "move tick".
			// if a segment has been on the same coordinate for the same amount of moves
			// as the snake has segments, time to move it up behind the snake head!
			// using > instead of >= because of 1-indexed looping.
			if segment.move_counter > snake_data.segments {
				// we can mark the current coordinate as free before moving:
				play_data.snakes_walkable_mask.set(&segment.coordinate, true);
				// fill the free spot behind the snake head:
				segment.coordinate = snake_data.previous_coordinate;
				segment.move_counter = 1; // we loop from 1 index to avoid segments getting into lockstep.
			} else { continue; }

			if snake_data.had_a_snack { 
				// skip one transform update turn when we have a newly spawned segment - 
				// it will appear behind the snake head when that moves.
				continue;
			}

			transform.translation = if snake_data.falling {
				Vec3::new(
					segment.coordinate.x as f32,
					SNAKE_Y - snake_data.fall_duration as f32, 
					segment.coordinate.y as f32
				)
			} else {
				Vec3::new(
					segment.coordinate.x as f32,
					SNAKE_Y,
					segment.coordinate.y as f32
				)
			};
		}

		// data reset when done.
		play_data.snake1_data.refresh_segments = false;
		play_data.snake1_data.had_a_snack = false;
		play_data.snake2_data.refresh_segments = false;
		play_data.snake2_data.had_a_snack = false;
		play_data.snake3_data.refresh_segments = false;
		play_data.snake3_data.had_a_snack = false;
	}
}

fn despawn_segments(
	game_state: ResMut<GameState>,
	query: Query<Entity, With<Segment>>,
	mut commands: Commands,
) {
	if let GameStateData::Reset(_counter) = game_state.data {
		for entity in query {
			commands.entity(entity).despawn();
		}
	}
}

fn is_opposite_direction(a: &Direction, b: &Direction) -> bool {
	match (a, b) {
		(Direction::Up, Direction::Down) => { true }
		(Direction::Down, Direction::Up) => { true }
		(Direction::Left, Direction::Right) => { true }
		(Direction::Right, Direction::Left) => { true }
		(_, _) => { false }
	}
}

fn evaluate_all_falling(
	mut game_state: ResMut<GameState>,
	query: Query<&mut Snake>,
) {
	if let GameStateData::Play(play_data) = &mut game_state.data {
		println!("all falling?");
		let mut all_falling = true;
		for snake in &query {
			if snake.active && !snake.falling {
				all_falling = false;
				break;
			}
		}
		play_data.all_falling = all_falling;
	}
}
