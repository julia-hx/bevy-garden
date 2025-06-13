use bevy::{input::keyboard::KeyboardInput, prelude::*};
use crate::state:: { GameStateData, GameStateEvent };

const HEAD_SIZE: f32 = 0.88;
const SNAKE_Y: f32 = 1.4;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, init_snakes);
		app.add_systems(Update,
			(read_gamestate_events, read_input, move_snakes).chain()
		);
	}
}

#[derive(Component)]
pub struct Snake {
	pub id: u32,
	pub direction: Direction,
	pub positions: Vec<Vec3>,
	pub last_move_time: f32,
	pub move_interval: f32,
	gamestate: GameStateData, // TODO: make gamestate data resource
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
	fn new(id: u32) -> Self {
		Self {
			id: id,
			direction: Direction::Up,
			positions: vec![],
			last_move_time: 0.0,
			move_interval: 1.0,
			gamestate: GameStateData::Init,
		}
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
		Snake::new(1),
		Direction::Up,
		InputMapping::new(
			KeyCode::ArrowUp,
			KeyCode::ArrowDown,
			KeyCode::ArrowLeft,
			KeyCode::ArrowRight),
		Transform::from_xyz(0.0, SNAKE_Y, 0.0),
		Mesh3d(meshes.add(Cuboid::new(HEAD_SIZE, HEAD_SIZE, HEAD_SIZE))),
		MeshMaterial3d(materials.add(Color::srgb_u8(80, 220, 220))),
	));
}

fn read_gamestate_events(
	mut gamestate_events: EventReader<GameStateEvent>,
	mut query: Query<(&mut Snake, &mut Transform)>,
) {
	let mut event_received = false;
	let mut event_data: &GameStateData = &GameStateData::Init;
	
	for e in gamestate_events.read() {
		event_received = true;
		event_data = &e.data;
		break;
	}

	if !event_received { return; }

	for (mut snake, mut transform) in &mut query {
		snake.gamestate = event_data.clone(); // TODO: make resource
		
		match snake.gamestate {
			GameStateData::Init => {},
			GameStateData::Setup (setup_data)=> {
				snake.move_interval = setup_data.move_interval;
			},
			GameStateData::Start => {},
			GameStateData::Play => {},
			GameStateData::Win => {},
			GameStateData::Death => {},
		}
	}
}

fn read_input(
	mut key_events: EventReader<KeyboardInput>,
	mut query: Query<(&mut Snake, &mut InputMapping)>,
) {
	for e in key_events.read() {
		for (mut snake, input_mapping) in &mut query {
			if e.key_code == input_mapping.up { snake.direction = Direction::Up; }
			else if e.key_code == input_mapping.down { snake.direction = Direction::Down; }
			else if e.key_code == input_mapping.left { snake.direction = Direction::Left; }
			else if e.key_code == input_mapping.right { snake.direction = Direction::Right; }
		}
	}
} 

fn move_snakes(
	time: Res<Time>,
	mut query: Query<(&mut Snake, &mut Transform)>,
) {
	for(mut snake, mut transform) in &mut query {
		if snake.last_move_time + snake.move_interval >= time.elapsed_secs() { continue; }

		let mut x = transform.translation.x;
		let mut z = transform.translation.z;
		
		match snake.direction {
			Direction::Up => { z -= 1.0; }
			Direction::Down => { z += 1.0; }
			Direction::Left => { x -= 1.0; }
			Direction::Right => { x += 1.0; }
		}

		*transform = Transform::from_xyz(x, SNAKE_Y, z);
		snake.last_move_time = time.elapsed_secs();
		println!("move {}", snake.last_move_time);
	}
}
