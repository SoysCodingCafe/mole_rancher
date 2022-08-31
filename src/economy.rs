use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::{start_menu::despawn_screen, GameState, reactor::GodMode};

// ---------- PLUGINS ----------
pub struct EconomyPlugin;

impl Plugin for EconomyPlugin {
	fn build(&self, app: &mut App) {
		app
		.add_system_set(SystemSet::on_enter(GameState::Reactor)
				.with_system(begin_day)
		)
		.add_system_set(SystemSet::on_exit(GameState::Reactor)
				.with_system(despawn_screen::<Economy>)
		)
		.add_system_set(SystemSet::on_pause(GameState::Reactor)
				.with_system(despawn_screen::<Economy>)
		)
		.add_system_set(SystemSet::on_resume(GameState::Reactor)
		)
		.add_system_set(SystemSet::on_update(GameState::Reactor)
				.with_system(tick_clock)
		)
		;
	}
}

// ---------- RESOURCES ----------
const START_POWER: f32 = 100.0;
pub const TARGET_POWER: f32 = 2000.0;

const START_PRESSURE: f32 = 0.0;
pub const MAX_PRESSURE: f32 = 50.0;
// Controls the averaging of the pressure [0-1]
// too high values will lead to the pressure bouncing around unpredictably and killing the player instantly
// too low values will make the thermometer lag behind whatever the real value
pub const PRESSURE_SENSITIVITY: f32 = 0.05;

// Game length in seconds
const DAY_LENGTH: f32 = 180.0;


// ---------- COMPONENTS ----------
#[derive(Component)]
pub struct Economy;

#[derive(Component, Inspectable)]
pub struct Power {
	pub current_power: f32,
	pub target_power: f32,
}

#[derive(Component, Inspectable)]
pub struct Temperature {
	pub current_pressure: f32,
	pub max_pressure: f32,
	pub thermometer_sprite: usize,
}

#[derive(Component)]
pub struct Clock {
	pub countdown: Timer,
	pub time_sprite: usize,
}

// ---------- SYSTEMS ----------
fn begin_day(
	mut commands: Commands,
) {
	commands
		.spawn()
		.insert(Name::new("Economy"))
		.insert(Economy)
		.insert(Power {
			current_power: START_POWER,
			target_power: TARGET_POWER,
		})
		.insert(Temperature {
			current_pressure: START_PRESSURE,
			max_pressure: MAX_PRESSURE,
			thermometer_sprite: 0,
		})
		.insert(Clock {
			countdown: Timer::from_seconds(DAY_LENGTH, false),
			time_sprite: 0,
		});
}

fn tick_clock (
	time: Res<Time>,
	mut clock_query: Query<(&Economy, &mut Clock, &Power)>,
	mut state: ResMut<State<GameState>>,
    godmode_query: Query<&GodMode>
) {
	let (_, mut clock, money) = clock_query.single_mut();
    let godmode = godmode_query.single();
    if !godmode.0 {
        if clock.countdown.tick(time.delta()).just_finished() {
            if money.current_power > money.target_power {
                state.push(GameState::WinScreen).expect("Failed to change states");
            }
            else {
                state.push(GameState::LoseScreen).expect("Failed to change states");
            }
        }
    }
}
