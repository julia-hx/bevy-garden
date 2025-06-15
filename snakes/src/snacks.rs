use bevy::prelude::*;
use crate::stage::{ StageEvent, StageEventData };

const SNACK_Y: f32 = 1.5;

pub struct SnacksPlugin;

impl Plugin for SnacksPlugin {
	fn build(&self, app: &mut App) {
		// app.add_systems(Startup, ());
		app.add_systems(Update, (read_stage_events, update_snacks).chain());
	}
}

#[derive(Component)]
pub struct SnackBar {
	foo: u32,
}

#[derive(Component)]
pub struct Snack {
	rotate_speed: f32,
}

fn ini_snacks() {

}

fn read_stage_events(
	mut stage_events: EventReader<StageEvent>,
	mut commands: Commands,
) {
	
}

fn update_snacks(
	query: Query<(&mut Snack, &mut Transform)>,
) {

}
