use bevy::{input::keyboard::KeyboardInput, prelude::*};
use crate::state::{ GameState, GameStateData, GameStateEvent };
use crate::stage::{ StageEvent, StageEventData };

const HEAD_SIZE: f32 = 1.0;
const SNAKE_Y: f32 = 1.4;
const DEFAULT_MOVE_INTERVAL: f32 = 0.6;
const HIDDEN_TRANSLATION: Vec3 = Vec3::new(1_000.0, 1_000.0, 1_000.0);

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
	pub segments: Vec<Vec3>,
	last_move_time: f32,
	move_interval: f32,
	spawn_point: Vec3,
	pub activated: bool,
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
			segments: vec![],
			last_move_time: 0.0,
			move_interval: 1.0,
			spawn_point: HIDDEN_TRANSLATION,
			activated: activated,
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
		Transform::from_xyz(HIDDEN_TRANSLATION.x, HIDDEN_TRANSLATION.y, HIDDEN_TRANSLATION.z),
		Mesh3d(meshes.add(Cuboid::new(HEAD_SIZE, HEAD_SIZE, HEAD_SIZE))),
		MeshMaterial3d(materials.add(Color::srgb_u8(80, 220, 220))),
	));

	commands.spawn((
		Snake::new(2, false),
		Direction::Up,
		InputMapping::new(
			KeyCode::ArrowUp,
			KeyCode::ArrowDown,
			KeyCode::ArrowLeft,
			KeyCode::ArrowRight),
		Transform::from_xyz(HIDDEN_TRANSLATION.x, HIDDEN_TRANSLATION.y, HIDDEN_TRANSLATION.z),
		Mesh3d(meshes.add(Cuboid::new(HEAD_SIZE, HEAD_SIZE, HEAD_SIZE))),
		MeshMaterial3d(materials.add(Color::srgb_u8(80, 220, 220))),
	));

	commands.spawn((
		Snake::new(3, false),
		Direction::Up,
		InputMapping::new(
			KeyCode::ArrowUp,
			KeyCode::ArrowDown,
			KeyCode::ArrowLeft,
			KeyCode::ArrowRight),
		Transform::from_xyz(HIDDEN_TRANSLATION.x, HIDDEN_TRANSLATION.y, HIDDEN_TRANSLATION.z),
		Mesh3d(meshes.add(Cuboid::new(HEAD_SIZE, HEAD_SIZE, HEAD_SIZE))),
		MeshMaterial3d(materials.add(Color::srgb_u8(80, 220, 220))),
	));
}

fn read_gamestate_events(
	mut gamestate_events: EventReader<GameStateEvent>,
	mut query: Query<(&mut Snake, &mut Transform)>,
) {
	let mut event_received = false;
	let mut gamestate_data = GameStateData::Init;
	
	for e in gamestate_events.read() {
		event_received = true;
		gamestate_data = e.data.clone();
		break;
	}

	if !event_received { return; }

	for (mut snake, mut transform) in &mut query {
		match gamestate_data {
			GameStateData::Init => {},
			GameStateData::Setup (setup_data)=> { 
				if setup_data.move_speed > 0.1 {
					snake.move_interval = DEFAULT_MOVE_INTERVAL / setup_data.move_speed; 
				} else {
					snake.move_interval = DEFAULT_MOVE_INTERVAL;
				}
			},
			GameStateData::Start => {
				if snake.activated {
					transform.translation = Vec3::new(snake.spawn_point.x, SNAKE_Y, snake.spawn_point.z);
				}
			},
			GameStateData::Play => {},
			GameStateData::Win => {},
			GameStateData::Death => {},
		}
	}
}

fn read_stage_events (
	mut stage_events: EventReader<StageEvent>,
	query: Query<&mut Snake>,
) {
	let mut event_received = false;
	let mut event_data = StageEventData::Empty;
	
	for e in stage_events.read() {
		match e.data {
			StageEventData::SetSnakeOrigin(_origin_data) => {
				event_received = true;
				event_data = e.data.clone();
			}
			_ => {}
		}
	}
	
	if !event_received { return; }

	for mut snake in query {
		match event_data {
			StageEventData::SetSnakeOrigin(origin_data) => {
				if origin_data.snake_id != snake.id { continue; }
				snake.spawn_point = origin_data.spawn_point;
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
			if e.key_code == input_mapping.up { snake.set_direction(Direction::Up); }
			else if e.key_code == input_mapping.down { snake.set_direction(Direction::Down); }
			else if e.key_code == input_mapping.left { snake.set_direction(Direction::Left); }
			else if e.key_code == input_mapping.right { snake.set_direction(Direction::Right); }
		}
	}
} 

fn move_snakes(
	time: Res<Time>,
	game_state: Res<GameState>,
	query: Query<(&mut Snake, &mut Transform)>,
) {
	match &game_state.data {
		GameStateData::Play => {}
		_ => { return; }
	}

	for(mut snake, mut transform) in query {
		if snake.last_move_time + snake.move_interval >= time.elapsed_secs() { continue; }

		let mut x = transform.translation.x;
		let mut z = transform.translation.z;
		
		match snake.direction {
			Direction::Up => { z -= 1.0; }
			Direction::Down => { z += 1.0; }
			Direction::Left => { x -= 1.0; }
			Direction::Right => { x += 1.0; }
		}

		transform.translation = Vec3::new(x, SNAKE_Y, z);
		snake.last_move_time = time.elapsed_secs();
	}
}

fn is_opposite_direction(a: &Direction, b: &Direction) -> bool {
	match (a, b) {
		(Direction::Up, Direction::Down) => { return true; }
		(Direction::Down, Direction::Up) => { return true; }
		(Direction::Left, Direction::Right) => { return true; }
		(Direction::Right, Direction::Left) => { return true; }
		(_, _) => { return false; }
	}
}
