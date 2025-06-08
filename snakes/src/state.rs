use bevy::prelude::*;

pub struct StatePlugin;

impl Plugin for StatePlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, init_gamestate);
		app.add_systems(Update, update_gamestate);
	}
}

enum GameStateData {
	Init,
	Setup,
	Start,
	Play,
	Win,
	Death,
}

#[derive(Bundle)]
struct GameStateBundle {
	data: GameState,
}

#[derive(Component)]
struct GameState {
	current_state: GameStateData,
}

impl GameState {
	fn new() -> Self {
		Self { current_state: GameStateData::Init }
	}

	fn set_gamestate(&mut self, target_state: GameStateData) {
		match target_state {
			GameStateData::Init => {
				println!("game state: Init");
			}
			GameStateData::Setup => {
				println!("game state: Setup");
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

		self.current_state = target_state;
	}
}

fn init_gamestate(mut commands: Commands) {
	println!("starting snakes game!");
	
	commands.spawn(GameStateBundle{
		data: GameState::new(),
	});
}

fn update_gamestate(mut query: Query<&mut GameState>, time: Res<Time>) {
	for (mut gs) in &mut query {
		match gs.current_state {
			GameStateData::Init => {
				gs.set_gamestate(GameStateData::Setup);
			}
			GameStateData::Setup => {
				
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
