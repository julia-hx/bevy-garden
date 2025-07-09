use bevy::prelude::*;
use rand::prelude::*;

use crate::stage::{ StageCoordinate };

const TUMBLE_MAX_TRANSLATION: i32 = 100;
const TUMBLE_MAX_ROTATION: i32 = 9;

pub struct AnimPlugin;

impl Plugin for AnimPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, (
			update_tumble_anims, update_oscillate_anims
		));
	}
}

#[derive(Component)]
pub struct TumbleAnim {
	translation: Vec3,
	rotation: Vec3,
	speed: f32,
}

impl TumbleAnim {
	pub fn new(speed: f32, unipolar_y: bool) -> Self {
		let y_min = if unipolar_y { 0 } else { -TUMBLE_MAX_TRANSLATION };
		let max = TUMBLE_MAX_TRANSLATION;
		let t_x = rand::random_range(-max..max) as f32 / 100.0;
		let t_y = rand::random_range(y_min..max) as f32 / 100.0;
		let t_z = rand::random_range(-max..max) as f32 / 100.0;

		let max = TUMBLE_MAX_ROTATION;
		let r_x = rand::random_range(-max..max) as f32;
		let r_y = rand::random_range(-max..max) as f32;
		let r_z = rand::random_range(-max..max) as f32;
		
		Self {
			translation: Vec3::new(t_x, t_y, t_z),
			rotation: Vec3::new(r_x, r_y, r_z),
			speed,
		}
	}
}

#[derive(Component)]
pub struct OscillateAnim {
	translation: Vec3,
	speed: Vec3,
	amplitude: Vec3,
}

impl OscillateAnim {
	pub fn new(translation: Vec3, speed: Vec3, amplitude: Vec3) -> Self { Self {translation, speed, amplitude} }
}

fn update_tumble_anims(
	time: Res<Time>,
	query: Query<(&mut Transform, &TumbleAnim)>,
) {
	for (mut transform, tumble) in query {
		transform.translation += tumble.translation * time.delta_secs() * tumble.speed;
		transform.rotate_x(tumble.rotation.x * time.delta_secs() * 0.5 * tumble.speed);
		transform.rotate_y(tumble.rotation.y * time.delta_secs() * 0.5 * tumble.speed);
		transform.rotate_z(tumble.rotation.z * time.delta_secs() * 0.5 * tumble.speed);
	}
}

fn update_oscillate_anims(
	time: Res<Time>,
	query: Query<(&mut Transform, &OscillateAnim)>,
) {
	for(mut transform, oscillate) in query {
		let offset = oscillate.translation.x;
		let x = oscillate.translation.x + (offset + time.elapsed_secs() * oscillate.speed.x).sin() * oscillate.amplitude.x * time.delta_secs();
		let y = oscillate.translation.y + (offset + time.elapsed_secs() * oscillate.speed.y).sin() * oscillate.amplitude.y * time.delta_secs();
		let z = oscillate.translation.z + (offset + time.elapsed_secs() * oscillate.speed.z).sin() * oscillate.amplitude.z * time.delta_secs();
		transform.translation = Vec3::new(x, y, z);
	}
}
