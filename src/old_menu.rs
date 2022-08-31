use bevy::{prelude::*, ui::FocusPolicy};

use crate::GameState;

// ---------- PLUGINS ----------
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
	fn build(&self, app: &mut App) {
		app
		.add_startup_system(generate_menu)
		.add_system(start_button)
		.add_system_set(SystemSet::on_pause(GameState::StartMenu)
			.with_system(close_menu)
		)
		.add_system_set(SystemSet::on_update(GameState::Game)
			.with_system(pause)
			.with_system(quit)
		)
		.add_system_set(SystemSet::on_update(GameState::PauseMenu)
			.with_system(un_pause)
		)
		;
	}
}

// ---------- RESOURCES ----------
struct UiAssets {
	font: Handle<Font>,
	button_idle: Handle<Image>,
	button_clicked: Handle<Image>,
}

// ---------- COMPONENTS ---------
#[derive (Component)]
pub struct ButtonHeld(bool);

// ---------- SYSTEMS ----------
fn generate_menu(
	mut commands: Commands,
	assets: Res<AssetServer>,
) {
	let ui_assets = UiAssets {
		font: assets.load("fonts/FiraSans-Bold.ttf"),
		button_idle: assets.load("button_idle.png"),
		button_clicked: assets.load("button_clicked.png")
	};
	commands
	.spawn_bundle(ButtonBundle {
		style: Style {
			align_self: AlignSelf::Center,
			align_items: AlignItems::Center,
			justify_content: JustifyContent::Center,
			size: Size::new(Val::Px(300.0), Val::Px(50.0)),
			margin: UiRect::all(Val::Auto),
			..Default::default()
		},
		// Make button transparent so button sprite is shown
		color: Color::NONE.into(),
		..Default::default()
	})
	.insert(ButtonHeld(false))
	.with_children(|parent| {
		parent.spawn_bundle( ImageBundle {
			style: Style {
				size: Size::new(Val::Percent(100.0) , Val::Percent(100.0)),
				justify_content: JustifyContent::Center,
				align_items: AlignItems::Center,
				..Default::default()
			},
			image: ui_assets.button_idle.clone().into(),
			..Default::default()
		})
		.insert(FocusPolicy::Pass)
		.with_children(|parent| {
			parent.spawn_bundle(
				TextBundle {
					text: Text::from_section(
						"Start Game",
						TextStyle {
							font: ui_assets.font.clone(),
							font_size: 40.0,
							color: Color::rgb(0.1, 0.8, 0.8),
					}),
					focus_policy: FocusPolicy::Pass,
					..Default::default()
			});
		});
	});
	commands.insert_resource(ui_assets);
}

fn start_button (
	mut interaction_query: Query<(&Children, &mut ButtonHeld, &Interaction), Changed<Interaction>>,
	mut image_query: Query<&mut UiImage>,
	ui_assets: Res<UiAssets>,
	mut state: ResMut<State<GameState>>,
) {
	for (children, mut button_held, interaction) in interaction_query.iter_mut() {
		let child = children.iter().next().unwrap();
		let mut image = image_query.get_mut(*child).unwrap();

		match interaction {
			Interaction::Clicked => {
				image.0 = ui_assets.button_clicked.clone();
				button_held.0 = true;
				state.push(GameState::Game).expect("Failed to change states");
			}
			Interaction::Hovered | Interaction::None => {
				image.0 = ui_assets.button_idle.clone();
			}
		}
	}
}

fn close_menu(
	mut commands: Commands,
	button_query: Query<Entity, With<Button>>,
) {
	for entity in button_query.iter() {
		commands.entity(entity).despawn_recursive();
	}

}

fn pause (
	mut keyboard: ResMut<Input<KeyCode>>,
	mut state: ResMut<State<GameState>>,
) {
	if keyboard.just_pressed(KeyCode::P) {
		state.push(GameState::PauseMenu).unwrap();
		keyboard.reset(KeyCode::P);
	}
}

fn un_pause (
	mut keyboard: ResMut<Input<KeyCode>>,
	mut state: ResMut<State<GameState>>,
) {
	if keyboard.just_pressed(KeyCode::P) {
		state.pop().unwrap();
		keyboard.reset(KeyCode::P);
	}
}

fn quit (
	mut keyboard: ResMut<Input<KeyCode>>,
	mut state: ResMut<State<GameState>>,
) {
	if keyboard.just_pressed(KeyCode::Q) {
		state.pop().unwrap();
		keyboard.reset(KeyCode::Q);
	}
}
