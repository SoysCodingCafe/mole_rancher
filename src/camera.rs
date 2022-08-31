use bevy::{prelude::*, input::mouse::MouseMotion};
use bevy_inspector_egui::Inspectable;

// ---------- PLUGINS ----------
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
	fn build(&self, app: &mut App) {
		app
		.add_startup_system(setup_camera)
		//.add_system(pan_camera)
		;
	}
}

// ---------- RESOURCES ----------

// ---------- COMPONENT ----------
#[derive(Component, Reflect, Inspectable)]
pub struct PanCamera {
	pub focus: Vec2,
}

// ---------- SYSTEMS ----------
fn setup_camera(
	mut commands: Commands
) {
	commands
	.spawn_bundle(Camera2dBundle::default())
	.insert(PanCamera {
		focus: Vec2::new(0.0, 0.0)
	});
}

fn pan_camera(
	mut motion: EventReader<MouseMotion>,
	mouse: Res<Input<MouseButton>>,
	mut query: Query<(&mut PanCamera, &mut Transform)>
) {
	let pan_button = MouseButton::Right;
	let mut pan_value = Vec2::ZERO;

	if mouse.pressed(pan_button) {
		for ev in motion.iter() {
			pan_value += ev.delta;
		}
	}

	for (mut pan, mut transform) in query.iter_mut() {
		if pan_value.length_squared() > 0.0 {
			pan.focus += pan_value;
			pan.focus.x = pan.focus.x.clamp(-400.0, 400.0);
			pan.focus.y = pan.focus.y.clamp(-400.0, 400.0);
			transform.translation.x = pan.focus.x;
			transform.translation.y = -pan.focus.y;
		}
	}
}
