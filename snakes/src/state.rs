use bevy::{input::keyboard::KeyboardInput, prelude::*};

use crate::stage;

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
	Win,
	Death,
}

// There is some duplication with GameStateEvent vs GameState as a resource.
// While one is used to actively trigger discrete global events,
// the other is passively available as a global resource - 
// but both are meant to always contain the same data.
// A better solution would be for the Resource to own the data, 
// and the events to propagate a cloneable or copyable ref.

#[derive(Event)]
pub struct GameStateEvent {
	pub data: GameStateData, // TODO: figure out good way to pass by ref
}

#[derive(Resource, Default)]
pub struct GameState {
	pub stage: u32,
	pub data: GameStateData,
}

impl GameState {
	fn set_data(&mut self, data: GameStateData, event_writer: &mut EventWriter<GameStateEvent>) {
		match data {
			GameStateData::Init => {
				println!("game state: Init");
			}
			GameStateData::Setup (setup_data) => {
				println!("game state: Setup stage {}", setup_data.stage_id);
			},
			GameStateData::Start => {
				println!("game state: Start");
			},
			GameStateData::Play (play_data) => {
				println!("game state: Play stage {} goal {}", play_data.stage_id, play_data.goal);
			},
			GameStateData::Win => {
				println!("game state: Win");
			},
			GameStateData::Death => {
				println!("game state: Death");
			},
		}

		event_writer.write(GameStateEvent { data: data.clone() });
		self.data = data;
	}
}

fn init_gamestate() {
	println!("starting snakes game!");
}

fn update_gamestate(mut event_writer: EventWriter<GameStateEvent>, 
	mut game_state: ResMut<GameState>,
	mut key_events: EventReader<KeyboardInput>,
) {
	let stage = game_state.stage;
	
	match game_state.data {
		GameStateData::Init => {
			game_state.stage = 0;
			let initial_setup_data = GameStateData::Setup(SetupData::new(stage));
			game_state.set_data(initial_setup_data, &mut event_writer);
		},
		GameStateData::Setup(setup_data) => {
			if setup_data.setup_done { game_state.set_data(GameStateData::Start, &mut event_writer); }
		},
		GameStateData::Start => {
			for e in key_events.read() {
				match e.key_code {
					KeyCode::Space => { game_state.set_data(GameStateData::Play(PlayData::new(stage)), &mut event_writer); }
					_ => {}
				}
			}
		}, 
		GameStateData::Play (play_data) => {
			if play_data.score >= play_data.goal {
				println!("Cleared stage {}!", play_data.stage_id);
			}
		},
		GameStateData::Win => {
			
		},
		GameStateData::Death => {
			
		}, 
	}
}

#[derive(Debug, Clone, Copy)]
pub struct SetupData {
	pub stage_id: u32,
	pub move_speed: f32,
	pub spotlight_translation: Vec3,
	pub spotlight_intensity_multiplier: f32,
	pub setup_done: bool,
}

impl SetupData {
	fn new(stage_id: u32) -> Self {
		Self {
			stage_id: stage_id,
			move_speed: 1.0,
			spotlight_translation: Vec3::new(6.0, 8.0, 4.0),
			spotlight_intensity_multiplier: 1.0,
			setup_done: false,
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub struct PlayData {
	pub stage_id: u32,
	pub goal: u32,
	pub score: u32,
}

impl PlayData {
	fn new(stage_id: u32) -> Self {
		Self {
			stage_id: stage_id,
			goal: 3,
			score: 0,
		}
	}
}
