use bevy::prelude::*;

pub struct StatePlugin;

impl Plugin for StatePlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, (init_gamestate));
		// app.add_systems(Update, ())
	}
}

enum GameState {
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

fn init_gamestate(mut commands: Commands) {
	println!("starting snakes game!");
	
	commands.spawn(GameStateBundle{
		data: GameStateData {
			current_state: GameState::Setup,
		},
	});

	set_gamestate(GameState::Setup); // TODO move this to game state entity struct
}

fn set_gamestate(target_state: GameState) {
	match target_state {
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
}
