use bevy::prelude::*;
use rand::prelude::*;

pub struct AnimPlugin;

impl Plugin for AnimPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, (update_tumble_anims));
	}
}

#[derive(Component)]
pub struct TumbleAnim {
	translation: Vec3,
	rotation: Vec3,
}

impl TumbleAnim {
	pub fn new() -> Self {
		let t_x = rand::random_range(-100..100) as f32 / 100.0;
		let t_y = rand::random_range(-100..100) as f32 / 100.0;
		let t_z = rand::random_range(-100..100) as f32 / 100.0;

		let r_x = rand::random_range(-6..6) as f32;
		let r_y = rand::random_range(-6..6) as f32;
		let r_z = rand::random_range(-6..6) as f32;
		
		Self {
			translation: Vec3::new(t_x, t_y, t_z),
			rotation: Vec3::new(r_x, r_y, r_z),
		}
	}
}

fn update_tumble_anims(
	time: Res<Time>,
	query: Query<(&mut Transform, &TumbleAnim)>,
) {
	for (mut transform, tumble) in query {
		transform.translation += tumble.translation * time.delta_secs();
		transform.rotate_x(tumble.rotation.x * time.delta_secs() * 0.5);
		transform.rotate_y(tumble.rotation.y * time.delta_secs() * 0.5);
		transform.rotate_z(tumble.rotation.z * time.delta_secs() * 0.5);
	}
}
