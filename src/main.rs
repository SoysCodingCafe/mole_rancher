// Disable Windows console on release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Import bevy!
use bevy::{prelude::*, render::texture::ImageSettings};

// ---------- PLUGINS ----------
// Modules
mod debug;
mod camera;
mod art;
mod audio;
mod start_menu;
mod lab;
mod reactor;
mod reactor_ui;
mod economy;
mod endgame;
mod enums;
mod logbook;

// Plugins
use debug::DebugPlugin;
use camera::CameraPlugin;
use art::ArtPlugin;
use audio::{AudioPlugin, Volume};
use start_menu::StartMenuPlugin;
use lab::LabPlugin;
use reactor::ReactorPlugin;
use reactor_ui::ReactorUiPlugin;
use economy::EconomyPlugin;
use endgame::EndgamePlugin;
use logbook::LogbookPlugin;

// Game State
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
	StartMenu,
	Lab,
	Reactor,
	Logbook,
	PauseMenu,
	SettingsMenu,
    SettingsAudioMenu,
	WinScreen,
	LoseScreen,
	BoomScreen,
	Credits
}

// ---------- RESOURCES ----------
// UI
pub const CLEAR: Color = Color::rgb(0.2, 0.8, 0.2);
pub const INVISIBLE: Color = Color::NONE;
pub const ASPECT_RATIO: f32 = 16.0 / 9.0;

fn main() {
	let height = 900.0;
	App::new()
	// Game States
	.add_state(GameState::StartMenu)
	// Resources
	.insert_resource(ImageSettings::default_nearest())
	.insert_resource(ClearColor(CLEAR))
	.insert_resource(WindowDescriptor {
		width: height * ASPECT_RATIO,
		height: height,
		// Working title
		title: "Mole Rancher".to_string(),
		present_mode: bevy::window::PresentMode::Fifo,
		// Maybe change later
		resizable: false,
		.. Default::default()
	})
    .insert_resource(Volume {
        bgm: 0.1,
        sfx: 0.1,
    })
	// Plugins
	.add_plugins(DefaultPlugins)
	.add_plugin(DebugPlugin)
	.add_plugin(CameraPlugin)
	.add_plugin(ArtPlugin)
	.add_plugin(AudioPlugin)
	.add_plugin(StartMenuPlugin)
	.add_plugin(LabPlugin)
	.add_plugin(ReactorPlugin)
	.add_plugin(ReactorUiPlugin)
	.add_plugin(EconomyPlugin)
	.add_plugin(EndgamePlugin)
    .add_plugin(LogbookPlugin)
	.run();
}
