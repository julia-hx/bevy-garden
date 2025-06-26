use bevy::{input::keyboard::KeyboardInput, prelude::*};
use crate::stage::{ StageCoordinate, StageWalkableMask};
use std::fs;

const LAST_STAGE: u32 = 3;
const STARTING_STAGE_PATH: &str = "./assets/save_data/starting_stage.txt";
const DEFAULT_MOVE_INTERVAL: f32 = 0.5;

pub struct StatePlugin;

impl Plugin for StatePlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<GameState>();
		app.add_systems(Startup, init_gamestate);
		app.add_systems(Update, update_gamestate);
		app.add_event::<GameStateEvent>();
	}
}

#[derive(Default, Debug, Clone)]
pub enum GameStateData {
	#[default]
	Init,
	Setup(SetupData),
	Start,
	Play(PlayData),
	Win(WinData),
	Death,
	Reset(u32), // counter for now
}

// First of all I wasn't aware of the State utils in bevy when starting to make this,
// so alot of the state flow here could be simplified using that
// and splitting "update" systems into smaller state specific systems.

// There is some duplication with GameStateEvent vs GameState as a resource.
// While one is used to actively trigger discrete global events,
// the other is passively available as a global resource - 
// but both are meant to always contain the same data.
// A better solution would be for the Resource to own the data, 
// and the events to propagate a ref - but what lifetime?

#[derive(Event)]
pub struct GameStateEvent {
	pub data: GameStateData, // TODO: pass by ref?
}

#[derive(Resource, Default)]
pub struct GameState {
	pub stage: u32,
	pub stage_width: usize,
	pub stage_height: usize,
	pub data: GameStateData,
}

impl GameState {
	fn set_data(&mut self, data: GameStateData, event_writer: &mut EventWriter<GameStateEvent>) {
		event_writer.write(GameStateEvent { data: data.clone() });
		self.data = data;

		match &self.data {
			GameStateData::Init => {
				println!("game state: Init");
			}
			GameStateData::Setup (setup_data) => {
				println!("game state: Setup stage {}", &setup_data.stage_id);
			},
			GameStateData::Start => {
				println!("game state: Start");
			},
			GameStateData::Play (play_data) => {
				println!("game state: Play stage {} goal {}", &play_data.stage_id, &play_data.goal);
			},
			GameStateData::Win (win_data) => {
				println!("game state: Win stage {}", &win_data.play_data.stage_id);
			},
			GameStateData::Death => {
				println!("game state: Death");
			},
			GameStateData::Reset(_counter) => {
				println!("game state: Reset");
			}
		}
	}
}

fn init_gamestate() {
	println!("starting snakes game!");
}

fn update_gamestate(
	mut event_writer: EventWriter<GameStateEvent>, 
	mut game_state: ResMut<GameState>,
	mut key_events: EventReader<KeyboardInput>,
) {
	match &mut game_state.data {
		GameStateData::Init => {
			let saved_stage = load_starting_stage();
			game_state.stage = if saved_stage <= LAST_STAGE { saved_stage } else { 0 };
			let initial_setup_data = GameStateData::Setup(SetupData::new(game_state.stage));
			game_state.set_data(initial_setup_data, &mut event_writer);
		}
		GameStateData::Setup(setup_data) => {
			for e in key_events.read() {
				if e.key_code == KeyCode::Space {
					setup_data.fast_forward = true;
				}
			}

			let go_to_start;

			if setup_data.fast_forward {
				go_to_start = setup_data.setup_done && setup_data.fast_forward_buffer >= 10; // insert a couple of "buffer" frames so input events don't overlap - tried key_events.clear()
				setup_data.fast_forward_buffer += 1;
			} else { go_to_start = setup_data.setup_done; }
			
			if go_to_start {
				game_state.set_data(GameStateData::Start, &mut event_writer); 
			}
		}
		GameStateData::Start => {
			for e in key_events.read() {
				if e.key_code == KeyCode::Space {
					let stage = game_state.stage;
					let width = game_state.stage_width;
					let height = game_state.stage_height;
					game_state.set_data(GameStateData::Play(PlayData::new(stage, width, height)), &mut event_writer);
				}
			}
		} 
		GameStateData::Play (play_data) => {
			if play_data.score >= play_data.goal {
				println!("Cleared stage {}!", play_data.stage_id);
				let win_data = WinData::new(play_data.clone());
				game_state.set_data(GameStateData::Win(win_data), &mut event_writer);
			} else if play_data.crash || play_data.all_falling {
				game_state.set_data(GameStateData::Death, &mut event_writer);
			}
		}
		GameStateData::Win (win_data) => {
			for e in key_events.read() {
				if e.key_code == KeyCode::Space {
					if game_state.stage < LAST_STAGE { game_state.stage += 1 };
					game_state.set_data(GameStateData::Reset(0), &mut event_writer);
				}
			}
		}
		GameStateData::Death => {
			for e in key_events.read() {
				if e.key_code == KeyCode::Space {
					game_state.set_data(GameStateData::Reset(0), &mut event_writer);
				}
			}
		}
		GameStateData::Reset(counter) => {
			*counter += 1;
			if *counter >= 30 {
				let stage = game_state.stage;
				game_state.set_data(GameStateData::Setup(SetupData::new(stage)), &mut event_writer);
			}
		}
	}
}

fn load_starting_stage() -> u32 {
	// liking rust here - this is so short and sweet!
	let savedata = fs::read_to_string(STARTING_STAGE_PATH).expect("starting stage save data not found!");
	let result = savedata.parse::<u32>().unwrap_or(0);
	result
}

#[derive(Debug, Clone, Copy)]
pub struct SetupData {
	pub stage_id: u32,
	pub spotlight_translation: Vec3,
	pub spotlight_intensity_multiplier: f32,
	pub setup_done: bool,
	pub fast_forward: bool,
	pub fast_forward_buffer: u32,
}

impl SetupData {
	fn new(stage_id: u32) -> Self {
		Self {
			stage_id,
			spotlight_translation: Vec3::new(6.0, 8.0, 4.0),
			spotlight_intensity_multiplier: 1.0,
			setup_done: false,
			fast_forward: false,
			fast_forward_buffer: 0,
		}
	}
}

#[derive(Debug, Clone)]
pub struct PlayData {
	pub stage_id: u32,
	pub goal: u32,
	pub score: u32,
	pub move_speed: f32,
	pub move_speed_increment: f32,
	pub move_interval: f32,
	pub last_move_time: f32,
	pub snake1_data: SnakePlayData,
	pub snake2_data: SnakePlayData,
	pub snake3_data: SnakePlayData,
	pub snakes_walkable_mask: StageWalkableMask,
	pub crash: bool,
	pub all_falling: bool,
}

impl PlayData {
	fn new(stage_id: u32, stage_width: usize, stage_height: usize) -> Self {
		let gameplay_config = GameplayConfig::new(stage_id);

		Self {
			stage_id,
			goal: gameplay_config.goal,
			score: 0,
			move_speed: gameplay_config.start_speed, // 1.0 = default snake speed set in snake.rs
			move_speed_increment: gameplay_config.speed_increment,
			move_interval: DEFAULT_MOVE_INTERVAL / gameplay_config.start_speed,
			last_move_time: 0.0,
			snake1_data: SnakePlayData::new(),
			snake2_data: SnakePlayData::new(),
			snake3_data: SnakePlayData::new(),
			snakes_walkable_mask: StageWalkableMask::new(stage_width, stage_height),
			crash: false,
			all_falling: false,
		}
	}

	pub fn increment_speed(&mut self) {
		self.move_speed += self.move_speed_increment;
		self.move_interval = if self.move_speed > 0.01 { DEFAULT_MOVE_INTERVAL / self.move_speed } else { DEFAULT_MOVE_INTERVAL / 0.01 };
	}
}

#[derive(Debug, Clone)]
pub struct SnakePlayData {
	pub active: bool,
	pub coordinate: StageCoordinate,
	pub previous_coordinate: StageCoordinate,
	pub had_a_snack: bool,
	pub falling: bool,
	pub fall_duration: u32,
	pub segments: u32,
	pub refresh_segments: bool,
	pub evaluate_move: bool,
}	

impl SnakePlayData {
	fn new() -> Self {
		SnakePlayData { 
			active: false,
			coordinate: StageCoordinate::new(0,0),
			previous_coordinate: StageCoordinate::new(0,0),
			had_a_snack: false,
			falling: false,
			fall_duration: 0,
			segments: 0,
			refresh_segments: false,
			evaluate_move: false,
		}
	}
}

#[derive(Debug, Clone)]
struct GameplayConfig {
	goal: u32,
	start_speed: f32,
	speed_increment: f32,
}

impl GameplayConfig {
	fn new(stage_id: u32) -> Self {
		match stage_id { // TODO: shared config storage
			0 => { Self { goal: 1, start_speed: 1.0, speed_increment: 0.1 } }
			1 => { Self { goal: 5, start_speed: 1.0, speed_increment: 0.12 } }
			2 => { Self { goal: 24, start_speed: 1.8, speed_increment: 0.04 } }
			3 => { Self { goal: 12, start_speed: 3.0, speed_increment: 0.1 } }
			_ => { Self { goal: 10, start_speed: 1.0, speed_increment: 0.05 } }
		}
	}
}

#[derive(Debug, Clone)]
pub struct WinData {
	pub play_data: PlayData, // data from the stage that was won
}

impl WinData {
	fn new(play_data: PlayData) -> Self {
		Self {
			play_data,
		}
	}
}
