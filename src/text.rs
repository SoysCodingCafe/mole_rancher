use bevy::prelude::*;

use crate::GameState;

// ---------- PLUGINS ----------
pub struct TextPlugin;

impl Plugin for TextPlugin {
	fn build(&self, app: &mut App) {
		app
		.add_startup_system(generate_text)
		.add_system_set(SystemSet::on_pause(GameState::StartMenu)
			.with_system(hide_text)
		)
		.add_system_set(SystemSet::on_resume(GameState::StartMenu)
			.with_system(show_text)
		)
		;
	}
}

// ---------- RESOURCES ----------


// ---------- COMPONENTS ---------
#[derive(Component)]
struct FpsText;

// ---------- SYSTEMS ----------
fn generate_text (
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	commands
	.spawn_bundle(
		TextBundle::from_section(
			"Mole Rauncher",
			TextStyle { 
				font: asset_server.load("fonts/FiraSans-Bold.ttf"), 
				font_size: 100.0, 
				color: Color::WHITE 
			},
		)
		.with_text_alignment(TextAlignment::TOP_CENTER)
		.with_style(Style { 
			align_self: AlignSelf::Center,
			position_type: PositionType::Absolute,
			position: UiRect {
				top: Val::Px(50.0),
				left: Val::Px(800.0 - 565.833 / 2.0), // 565.833 is calculate text width
				..default()
			},
			..default()
		}),
	)
	.insert(FpsText);
}

fn hide_text (
	mut query: Query<&mut Text, With<FpsText>>
) {
	for mut text in &mut query {
		text.sections[0].style.font_size = 0.0;
	}
}

fn show_text (
	mut query: Query<&mut Text, With<FpsText>>
) {
	for mut text in &mut query {
		text.sections[0].style.font_size = 100.0;
	}
}
