use bevy::prelude::*;

use crate::{GameState, start_menu::despawn_screen, audio::LoopTimer, reactor::MoleculeList, economy::Economy};

// ---------- PLUGINS ----------
pub struct EndgamePlugin;


// Can probably simplify this whole thing by having a "winstate" resource 
// and changing the sprite in a single system
impl Plugin for EndgamePlugin {
	fn build(&self, app: &mut App) {
		app
		.add_system_set(SystemSet::on_enter(GameState::WinScreen)
			.with_system(generate_winscreen)
			.with_system(generate_endgame)
            .with_system(despawn_screen::<MoleculeList>)
            .with_system(despawn_screen::<Economy>)
		)
		.add_system_set(SystemSet::on_pause(GameState::WinScreen)
			.with_system(despawn_screen::<EndgameSprites>)
			.with_system(despawn_screen::<EndgameTimer>)
		)
		.add_system_set(SystemSet::on_update(GameState::WinScreen)
			.with_system(cut_to_credits)
		)

		.add_system_set(SystemSet::on_enter(GameState::LoseScreen)
			.with_system(generate_losescreen)
			.with_system(generate_endgame)
            .with_system(despawn_screen::<MoleculeList>)
            .with_system(despawn_screen::<Economy>)
		)
		.add_system_set(SystemSet::on_pause(GameState::LoseScreen)
		.with_system(despawn_screen::<EndgameSprites>)
		.with_system(despawn_screen::<EndgameTimer>)
		)
		.add_system_set(SystemSet::on_update(GameState::LoseScreen)
			.with_system(cut_to_credits)
		)

		.add_system_set(SystemSet::on_enter(GameState::BoomScreen)
			.with_system(generate_boomscreen)
			.with_system(generate_endgame)
            .with_system(despawn_screen::<MoleculeList>)
		)
		.add_system_set(SystemSet::on_pause(GameState::BoomScreen)
			.with_system(despawn_screen::<EndgameSprites>)
			.with_system(despawn_screen::<EndgameTimer>)
		)
		.add_system_set(SystemSet::on_update(GameState::BoomScreen)
		.with_system(cut_to_credits)
		)

		.add_system_set(SystemSet::on_enter(GameState::Credits)
			.with_system(generate_credits)
		)
		.add_system_set(SystemSet::on_exit(GameState::Credits)
			.with_system(despawn_screen::<CreditsSprites>)
			.with_system(despawn_screen::<EndgameTimer>)
		)
		.add_system_set(SystemSet::on_update(GameState::Credits)
			.with_system(return_to_menu)
		)
		;
	}
}

// ---------- COMPONENTS ----------
#[derive(Component)]
struct EndgameSprites;

#[derive(Component)]
struct CreditsSprites;

#[derive(Component)]
pub struct BasicCountdown(pub Timer);

#[derive(Component)]
struct EndgameTimer;

// ---------- SYSTEMS ----------
fn generate_endgame (
	mut commands: Commands,
) {
	commands
		.spawn()
		.insert(Name::new("Endgame Timer"))
		.insert(BasicCountdown(Timer::from_seconds(10.0, false)))
        .insert(EndgameTimer)
		;
}

fn generate_winscreen (
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("endgame/winscreen.png"),
            transform: Transform::from_scale(Vec3::new(4.0, 4.0, 0.0)),
			..default()
	})
	.insert(Name::new("Winscreen Sprite"))
	.insert(EndgameSprites)
	;
}

fn generate_losescreen (
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("endgame/losescreen.png"),
			transform: Transform::from_scale(Vec3::new(4.0, 4.0, 0.0)),
			..default()
	})
	.insert(Name::new("Losescreen Sprite"))
	.insert(EndgameSprites)
	;
}

fn generate_boomscreen (
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("endgame/boomscreen.png"),
			transform: Transform::from_scale(Vec3::new(4.0, 4.0, 0.0)),
			..default()
	})
	.insert(Name::new("Boomscreen Sprite"))
	.insert(EndgameSprites)
	;
}

fn cut_to_credits (
	time: Res<Time>,
	mut endtime_query: Query<&mut BasicCountdown, Without<LoopTimer>>,
	mut state: ResMut<State<GameState>>,
) {
	let mut endtime = endtime_query.single_mut();

	if endtime.0.tick(time.delta()).just_finished() {
		state.push(GameState::Credits).expect("Failed to change states");
	}
}

fn generate_credits (
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("endgame/credits.png"),
            transform: Transform::from_scale(Vec3::new(4.0, 4.0, 0.0)),
			..default()
		})
		.insert(Name::new("Credits Sprite"))
		.insert(CreditsSprites)
		;

	commands
		.spawn()
		.insert(Name::new("Credits Timer"))
		.insert(BasicCountdown(Timer::from_seconds(10.0, false)))
        .insert(EndgameTimer)
		;
}

fn return_to_menu (
	time: Res<Time>,
	mut creditstime_query: Query<&mut BasicCountdown, Without<LoopTimer>>,
	mut state: ResMut<State<GameState>>,
) {
	let mut creditstime = creditstime_query.single_mut();

	if creditstime.0.tick(time.delta()).just_finished() {
		state.set(GameState::StartMenu).expect("Failed to change states");
	}
}
