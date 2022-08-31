use bevy::{prelude::*, audio::AudioSink};
use bevy_inspector_egui::Inspectable;

use crate::{GameState, start_menu::despawn_screen, endgame::BasicCountdown};

// ---------- PLUGINS ----------
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
	fn build(&self, app: &mut App) {
		app
		.add_startup_system(start_bgm)
        .add_system(loop_bgm)
        .add_system(update_audio_volume)
		.add_system_set(SystemSet::on_enter(GameState::StartMenu)
			//.with_system(start_bgm)
			//.with_system(load_ding)
		)
		.add_system_set(SystemSet::on_pause(GameState::StartMenu)
			//.with_system(pause_bgm)
			//.with_system(despawn_screen::<Volume>)
		)
		.add_system_set(SystemSet::on_resume(GameState::StartMenu)
			//.with_system(start_bgm)
		)
		.add_system_set(SystemSet::on_update(GameState::StartMenu)
			//.with_system(update_audio_volume)
		)
		;
	}
}

// ---------- RESOURCES ----------
const BGM_VOLUME: f32 = 0.1;
const SFX_VOLUME: f32 = 0.1;

// Assets
pub struct Sfx(pub Handle<AudioSource>);
pub struct Bgm(pub Handle<AudioSink>);

#[derive(Component)]
pub struct LoopTimer;

#[derive(Component, PartialEq, Copy, Clone, Inspectable)]
pub struct Volume{
	pub bgm: f32,
	pub sfx: f32
}

// ---------- SYSTEMS ----------
fn load_ding (
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	let ding_source: Handle<AudioSource> = asset_server.load("audio/ding.ogg");
	commands.insert_resource(Sfx(ding_source));
}

fn start_bgm (
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	audio: Res<Audio>,
	audio_sinks: Res<Assets<AudioSink>>,
) {
	let bgm_source: Handle<AudioSource> = asset_server.load("audio/boop_beep.ogg");
	let bgm_sink = audio_sinks.get_handle(audio.play(bgm_source));
	commands.insert_resource(Bgm(bgm_sink));

	commands
		.spawn()
        // Song length is 180 seconds
        .insert(BasicCountdown(Timer::from_seconds(180.0, true)))
        .insert(LoopTimer)
		.insert(Name::new("Audio Repeat"))
		;
}

fn loop_bgm (
    mut commands: Commands,
    time: Res<Time>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    mut loop_time_query: Query<&mut BasicCountdown, With<LoopTimer>>,
    audio_sinks: Res<Assets<AudioSink>>,
) {
    let mut loop_time = loop_time_query.single_mut();

	if loop_time.0.tick(time.delta()).just_finished() {
        let bgm_source: Handle<AudioSource> = asset_server.load("audio/boop_beep.ogg");
	    let bgm_sink = audio_sinks.get_handle(audio.play(bgm_source));
	    commands.insert_resource(Bgm(bgm_sink));
    }
}

fn pause_bgm (
	audio_sinks: Res<Assets<AudioSink>>,
	bgm: Res<Bgm>,
) {
	if let Some(sink) = audio_sinks.get(&bgm.0) {
		if !sink.is_paused() {
			sink.pause();
		}
	}
}

fn play_bgm (
	audio_sinks: Res<Assets<AudioSink>>,
	bgm: Res<Bgm>,
) {
	if let Some(sink) = audio_sinks.get(&bgm.0) {
		if sink.is_paused() {
			sink.play();
		}
	}
}

fn update_audio_volume (
	audio_sinks: Res<Assets<AudioSink>>,
	volume: Res<Volume>,
	bgm: Res<Bgm>,
	//sfx: Res<SfX>,
) {
	if let Some(sink) = audio_sinks.get(&bgm.0) {
		sink.set_volume(volume.bgm);
	}
	/*if let Some(sink) = audio_sinks.get(&sfx.0) {
		sink.set_volume(volume.sfx);
	}*/
}
