use bevy::prelude::*;
use crate::stage::{ StageEvent, StageEventData };

const SNACK_Y: f32 = 1.4;

pub struct SnacksPlugin;

impl Plugin for SnacksPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, (read_stage_events, update_snacks).chain());
	}
}

#[derive(Component)]
pub struct Snack {
	rotate_speed: f32,
}

fn read_stage_events(
	mut stage_events: EventReader<StageEvent>,
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	query: Query<Entity, With<Snack>>
) {
	for e in stage_events.read() {
		match e.data {
			StageEventData::SpawnSnack(spawn_point) => {
				for entity in query {
					commands.entity(entity).despawn();
				}
				println!("...spawning next snack!");
				commands.spawn((
					Mesh3d(meshes.add(Tetrahedron::default())),
					MeshMaterial3d(materials.add(Color::srgb_u8(220, 220, 100))),
					Transform {
						translation: Vec3 { x: spawn_point.x as f32, y: SNACK_Y, z: spawn_point.y as f32 },
						rotation: Quat::IDENTITY,
						scale: Vec3::new(0.6, 0.6, 0.6),
					},
					Snack {rotate_speed: 1.0},
				));
			}
			_ => {}
		}
	}
}

fn update_snacks(
	time: Res<Time>,
	query: Query<(&Snack, &mut Transform)>,
) {
	for (snack, mut transform) in query {
		transform.rotate_x((time.delta_secs() / 5.) * snack.rotate_speed );
		transform.rotate_y((time.delta_secs() / 2.) * snack.rotate_speed );
		transform.rotate_z((time.delta_secs() / 3.) * snack.rotate_speed );
	}
}
