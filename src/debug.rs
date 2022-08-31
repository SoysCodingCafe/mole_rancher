use bevy::prelude::*;
use bevy_inspector_egui::{WorldInspectorPlugin, RegisterInspectable};

use crate::{reactor::{Molecule, Dimensions, Velocity, Unlocked}, camera::PanCamera, economy::{Power, Temperature}, audio::Volume};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
	fn build(&self, app: &mut App) {
		if cfg!(debug_assertions) {
			app
			.add_plugin(WorldInspectorPlugin::new())
			.register_inspectable::<Molecule>()
			.register_inspectable::<Dimensions>()
			.register_inspectable::<Velocity>()
			.register_inspectable::<Power>()
			.register_inspectable::<Temperature>()
			.register_inspectable::<PanCamera>()
            .register_inspectable::<Volume>()
            .register_inspectable::<Unlocked>()
			;
		}
	}
}
