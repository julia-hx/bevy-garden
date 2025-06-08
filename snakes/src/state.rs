use bevy::prelude::*;

pub struct StatePlugin;

impl Plugin for StatePlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, init_gamestate);
		app.add_systems(Update, update_gamestate);
	}
}

enum GameState {
	Init,
	Setup,
	Start,
	Play,
	Win,
	Death,
}

#[derive(Bundle)]
struct GameStateBundle {
	data: GameStateData,
}

#[derive(Component)]
struct GameStateData {
	current_state: GameState,
}

impl GameStateData {
	fn new() -> Self {
		Self { current_state: GameState::Init }
	}

	fn set_gamestate(&mut self, target_state: GameState) {
		match target_state {
			GameState::Init => {
				println!("game state: Init");
			}
			GameState::Setup => {
				println!("game state: Setup");
			},
			GameState::Start => {
				println!("game state: Start");
			},
			GameState::Play => {
				println!("game state: Play");
			},
			GameState::Win => {
				println!("game state: Win");
			},
			GameState::Death => {
				println!("game state: Death");
			},
		}

		self.current_state = target_state;
	}
}

fn init_gamestate(mut commands: Commands) {
	println!("starting snakes game!");
	
	commands.spawn(GameStateBundle{
		data: GameStateData::new(),
	});
}

fn update_gamestate(mut query: Query<&mut GameStateData>, time: Res<Time>) {
	for (mut gs) in &mut query {
		match gs.current_state {
			GameState::Init => {
				gs.set_gamestate(GameState::Setup);
			}
			GameState::Setup => {
				
			},
			GameState::Start => {
				
			},
			GameState::Play => {
				
			},
			GameState::Win => {
				
			},
			GameState::Death => {
				
			},
		}
	}
}
