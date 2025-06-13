use bevy::prelude::*;

pub struct StatePlugin;

impl Plugin for StatePlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, init_gamestate);
		app.add_systems(Update, update_gamestate);
		app.add_event::<GameStateEvent>();
	}
}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
pub enum GameStateData {
	Init,
	Setup(SetupData),
	Start,
	Play,
	Win,
	Death,
}

#[derive(Event)]
pub struct GameStateEvent {
	pub data: GameStateData, // TODO: figure out good way to pass by ref
}

#[derive(Bundle)]
struct GameStateBundle {
	game_state: GameState,
}

#[derive(Component)]
struct GameState {
	data: GameStateData,
}

#[derive(Resource)]
struct GameSateResource {
	data: GameStateData,
}

impl GameState {
	fn new() -> Self {
		Self {
			data: GameStateData::Init,
		}
	}

	fn set_gamestate(&mut self, target_state: GameStateData, event_writer: &mut EventWriter<GameStateEvent>) {
		match target_state {
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

		event_writer.write(GameStateEvent { data: target_state.clone() }); // TODO: no clone plz
		self.data = target_state;
	}
}

fn init_gamestate(mut commands: Commands) {
	println!("starting snakes game!");
	
	commands.spawn(GameStateBundle{
		game_state: GameState::new(),
	});
}

fn update_gamestate(mut event_writer: EventWriter<GameStateEvent>, 
	mut query: Query<&mut GameState>,
) {
	for mut gs in &mut query {
		match gs.data {
			GameStateData::Init => {
				let initial_setup_data = GameStateData::Setup(SetupData::new());
				gs.set_gamestate(initial_setup_data, &mut event_writer);
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
}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
pub struct SetupData {
	pub stage_id: u32,
	pub move_interval: f32,
}

impl SetupData {
	fn new() -> Self {
		Self {
			stage_id: 0,
			move_interval: 0.33,
		}
	}
}
