use bevy::{input::keyboard::KeyboardInput, prelude::*};
use crate::state::{ GameState, GameStateData, GameStateEvent };
use crate::stage::{ StageCoordinate, StageEvent, StageEventData };

const SNAKE_HEAD_SIZE: Vec3 = Vec3::new(1.0, 0.8, 1.0);
const SNAKE_Y: f32 = 1.4;
const DEFAULT_MOVE_INTERVAL: f32 = 0.6;
const HIDDEN_COORDINATE: StageCoordinate = StageCoordinate::new(1000, 1000);

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, init_snakes);
		app.add_systems(Update,
			(read_gamestate_events, read_stage_events, read_input, move_snakes).chain()
		);
	}
}

#[derive(Component)]
pub struct Snake {
	pub id: u32,
	pub direction: Direction,
	pub falling: bool,
	pub fall_duration: u32,
	pub segments: Vec<Vec3>,
	last_move_time: f32,
	move_interval: f32,
	stage_coordinate: StageCoordinate,
	pub activated: bool,
	pub input_received: bool,
}

#[derive(Component)]
pub struct InputMapping {
	pub up: KeyCode,
	pub down: KeyCode,
	pub left: KeyCode,
	pub right: KeyCode,
}

#[derive(Component)]
pub enum Direction {
	Up,
	Down,
	Left,
	Right,
}

impl Snake {
	fn new(id: u32, activated: bool) -> Self {
		Self {
			id: id,
			direction: Direction::Up,
			falling: false,
			fall_duration: 0,
			segments: vec![],
			last_move_time: 0.0,
			move_interval: 1.0,
			stage_coordinate: HIDDEN_COORDINATE,
			activated: activated,
			input_received: false,
		}
	}

	fn set_direction(&mut self, direction: Direction) {
		if is_opposite_direction(&self.direction, &direction) {
			println!("-- snake can't turn around on itself!");
			return;
		}
		self.direction = direction;
	}
}

impl InputMapping {
	fn new(
		up: KeyCode, 
		down: KeyCode, 
		left: KeyCode, 
		right: KeyCode
	) -> Self {
		Self {
			up: up, down: down, left: left, right: right
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
		MeshMaterial3d(materials.add(Color::srgb_u8(80, 220, 220))),
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
		MeshMaterial3d(materials.add(Color::srgb_u8(80, 220, 220))),
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
		MeshMaterial3d(materials.add(Color::srgb_u8(80, 220, 220))),
	));
}

fn read_gamestate_events(
	mut gamestate_events: EventReader<GameStateEvent>,
	mut query: Query<(&mut Snake, &mut Transform)>,
) {
	let gamestate_data;
	
	if let Some(e) = gamestate_events.read().next() {
		gamestate_data = e.data.clone();
	} else { return; }

	for (mut snake, mut transform) in &mut query {
		match gamestate_data {
			GameStateData::Init => {},
			GameStateData::Setup (setup_data)=> { 
				if setup_data.move_speed > 0.1 {
					snake.move_interval = DEFAULT_MOVE_INTERVAL / setup_data.move_speed; 
				} else {
					snake.move_interval = DEFAULT_MOVE_INTERVAL;
				}
				snake.falling = false;
				snake.fall_duration = 0;
			},
			GameStateData::Start => {
				if snake.activated {
					transform.translation = Vec3::new(snake.stage_coordinate.x as f32, SNAKE_Y, snake.stage_coordinate.y as f32);
				}
			},
			GameStateData::Play (play_data) => {
				
			},
			GameStateData::Win => {},
			GameStateData::Death => {},
		}
	}
}

fn read_stage_events (
	mut stage_events: EventReader<StageEvent>,
	query: Query<&mut Snake>,
) {
	let event_data;
	
	if let Some (e) = stage_events.read().next() {
		event_data = e.data.clone();
	} else { return; }

	for mut snake in query {
		match event_data {
			StageEventData::SetSnakeSpawnPoint(spawn_point_data) => {
				if spawn_point_data.snake_id != snake.id { continue; }
				snake.stage_coordinate = spawn_point_data.spawn_point;
			}
			StageEventData::SnackEaten(snake_id) => {
				if snake_id == snake.id {
					println!("snake {} had a lil snack!", snake_id);
				}
			}
			StageEventData::SnakeFalling(snake_id) => {
				if snake_id == snake.id {
					snake.falling = true;
					println!("snake {} is falling!", snake_id);
				}
			}
			_ => {}
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
				if !snake.activated && snake.input_received { 
					snake.activated = true;
					transform.translation = Vec3::new(snake.stage_coordinate.x as f32, SNAKE_Y, snake.stage_coordinate.y as f32);
				}
			}
		}
		GameStateData::Play (play_data) => {
			for(mut snake, mut transform) in query {
				// no so fond of this...
				match snake.id {
					1 => { play_data.snake1_active = snake.activated; play_data.snake1_falling = snake.falling; }
					2 => { play_data.snake2_active = snake.activated; play_data.snake2_falling = snake.falling; }
					3 => { play_data.snake3_active = snake.activated; play_data.snake3_falling = snake.falling; }
					_=> ()
				}

				if !snake.activated { continue; }
				if snake.last_move_time + snake.move_interval >= time.elapsed_secs() { continue; }
				
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
					}

					match snake.id {
						1 => { play_data.snake1_coordinate = snake.stage_coordinate; }
						2 => { play_data.snake2_coordinate = snake.stage_coordinate; }
						3 => { play_data.snake3_coordinate = snake.stage_coordinate; }
						_=> ()
					}

					next_translation = Vec3::new(snake.stage_coordinate.x as f32, SNAKE_Y, snake.stage_coordinate.y as f32);
				}
				
				transform.translation = next_translation;
				snake.last_move_time = time.elapsed_secs();
			}
		}
		_ => { () }
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
