use bevy::prelude::*;

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
	Play,
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
			GameStateData::Play => {
				println!("game state: Play");
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
) {
	match game_state.data {
		GameStateData::Init => {
			let initial_setup_data = GameStateData::Setup(SetupData::new());
			game_state.set_data(initial_setup_data, &mut event_writer);
		},
		GameStateData::Setup(_setup_data) => {
			
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

#[derive(Debug, Clone, Copy)]
pub struct SetupData {
	pub stage_id: u32,
	pub move_interval: f32,
}

impl SetupData {
	fn new() -> Self {
		Self {
			stage_id: 0,
			move_interval: 0.6,
		}
	}
}
