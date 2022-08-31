use bevy::prelude::*;

use crate::art::{spawn_spritesheet_sprite, SpriteSheets};
use crate::economy::Economy;
use crate::enums::SpriteType;
use crate::reactor::{generate_molecule_list, MoleculeList};
use crate::{GameState, INVISIBLE};

use crate::start_menu::despawn_screen;

// ---------- PLUGINS ----------
pub struct LabPlugin;

impl Plugin for LabPlugin {
	fn build(&self, app: &mut App) {
		app
		.add_system_set(SystemSet::on_enter(GameState::Lab)
			.with_system(load_lab)
            .with_system(despawn_screen::<MoleculeList>)
            .with_system(despawn_screen::<Economy>)
		)
        .add_system_set(SystemSet::on_resume(GameState::Lab)
            .with_system(load_lab)
            .with_system(despawn_screen::<MoleculeList>)
            .with_system(despawn_screen::<Economy>)
        )
		.add_system_set(SystemSet::on_update(GameState::Lab)
		.with_system(update_lab_sprite)
			.with_system(button_reactor)
			.with_system(button_logbook)
			.with_system(button_exit)
		)
		.add_system_set(SystemSet::on_pause(GameState::Lab)
			.with_system(despawn_screen::<LabMenuUi>)
			.with_system(despawn_screen::<LabSprites>)
            .with_system(generate_molecule_list)
		)
		.add_system_set(SystemSet::on_exit(GameState::Lab)
			.with_system(despawn_screen::<LabMenuUi>)
			.with_system(despawn_screen::<LabSprites>)
            .with_system(generate_molecule_list)
		)
		;
	}
}

// ---------- RESOURCES ----------


// ---------- COMPONENTS ----------
#[derive(Component)]
struct LabSprites;

#[derive(Component)]
struct LabSpriteIndex(usize);

#[derive(Component)]
struct LabMenuUi;

#[derive(Component)]
struct ReactorButton;

#[derive(Component)]
struct LogbookButton;

#[derive(Component)]
struct ExitButton;

#[derive(Component)]
struct ExtinguisherButton;

// ---------- SYSTEMS ----------
fn load_lab (
	mut commands: Commands,
	spritesheet: Res<SpriteSheets>,
	asset_server: Res<AssetServer>,
) {   
	let lab = spawn_spritesheet_sprite(
		&mut commands,
		&spritesheet,
		SpriteType::Lab,
		0,
		Color::WHITE,
		//Color::rgb(random::<f32>(), random::<f32>(), random::<f32>()),
		// Find some way to pass Reactor to spawn molecule inside and bound movement within
		// Pass reactor index to function, use reactor index to find reactor size from transform.scale
		Vec3::new(0.0, 0.0,0.0),
		Vec2::new(1600.0, 900.0)
	);

	commands
		.entity(lab)
		.insert(Name::new("Lab Sprites"))
		.insert(LabSprites)
		.insert(LabSpriteIndex(0));
	
	// Test square for adjusting UI positioning
	/*commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("test_square.png"),
			transform: Transform::from_scale(Vec3::splat(1.0)),
			..default()
		})
		.insert(Name::new("Test Square"));*/

	commands
		.spawn_bundle(NodeBundle {
			style: Style {
				position_type: PositionType::Absolute,
				position: UiRect {
					// Screen width, sprite width, x translation
					left: Val::Px(1600.0 / 2.0 - 230.0 / 2.0 + 231.0),
					// Screen height, sprite height, y translation
					bottom: Val::Px(900.0 / 2.0 - 100.0 / 2.0 - 8.8), 
					..default()
				},
				// Sprite width and height
				size: Size::new(Val::Px(230.0), Val::Px(100.0)),
				..default()
			},
			color: INVISIBLE.into(),
			// Sprite rotation
			transform: Transform::from_rotation(Quat::from_rotation_z(-0.12)),
			..default()
		})
		.insert(Name::new("Reactor Node"))
		.insert(LabMenuUi)
		.with_children(|parent| {

			// Reactor Button
			parent
				.spawn_bundle(ButtonBundle {
					style: Style {
						size: Size::new(Val::Px(230.0), Val::Px(100.0)),
						..default()
					},
					color: INVISIBLE.into(),
					..default()
				})
				.insert(ReactorButton);
	});

	commands
		.spawn_bundle(NodeBundle {
			style: Style {
				position_type: PositionType::Absolute,
				position: UiRect {
					// Screen width, sprite width, x translation
					left: Val::Px(1600.0 / 2.0 - 320.0 / 2.0 - 290.8),
					// Screen height, sprite height, y translation
					bottom: Val::Px(900.0 / 2.0 - 120.0 / 2.0 - 174.5), 
					..default()
				},
				// Sprite width and height
				size: Size::new(Val::Px(320.0), Val::Px(120.0)),
				..default()
			},
			color: INVISIBLE.into(),
			// Sprite rotation
			transform: Transform::from_rotation(Quat::from_rotation_z(0.2)),
			..default()
		})
		.insert(Name::new("Logbook Node"))
		.insert(LabMenuUi)
		.with_children(|parent| {

			// Logbook Button
			parent
				.spawn_bundle(ButtonBundle {
					style: Style {
						size: Size::new(Val::Px(320.0), Val::Px(120.0)),
						..default()
					},
					color: INVISIBLE.into(),
					..default()
				})
				.insert(LogbookButton);
		});

	commands
		.spawn_bundle(NodeBundle {
			style: Style {
				position_type: PositionType::Absolute,
				position: UiRect {
					// Screen width, sprite width, x translation
					left: Val::Px(1600.0 / 2.0 - 320.0 / 2.0 - 750.3),
					// Screen height, sprite height, y translation
					bottom: Val::Px(900.0 / 2.0 - 650.0 / 2.0 + 26.2), 
					..default()
				},
				// Sprite width and height
				size: Size::new(Val::Px(320.0), Val::Px(650.0)),
				..default()
			},
			color: INVISIBLE.into(),
			// Sprite rotation
			transform: Transform::from_rotation(Quat::from_rotation_z(0.41)),
			..default()
		})
		.insert(Name::new("Exit Node"))
		.insert(LabMenuUi)
		.with_children(|parent| {

			// Exit Button
			parent
				.spawn_bundle(ButtonBundle {
					style: Style {
						size: Size::new(Val::Px(320.0), Val::Px(650.0)),
						..default()
					},
					color: INVISIBLE.into(),
					..default()
				})
				.insert(ExitButton);
		});

}

fn update_lab_sprite (
	lab_index_query: Query<&LabSpriteIndex>,
	mut lab_sprite_query: Query<(&LabSprites, &mut TextureAtlasSprite)>,
) {
	let index = lab_index_query.single();
	let (_, mut sprite) = lab_sprite_query.single_mut();

	sprite.index = index.0;
}

fn button_reactor (
	mut lab_query: Query<&mut LabSpriteIndex>,
	mut interaction_query: Query<(&ReactorButton, &Interaction), Changed<Interaction>>,
	mut state: ResMut<State<GameState>>,
) {
	let mut lab_index = lab_query.single_mut();
	for (_, interaction) in interaction_query.iter_mut() {
		match interaction {
			Interaction::Clicked => {
				state.push(GameState::Reactor).expect("Failed to change states");
			}
			Interaction::Hovered => {
				lab_index.0 = 2;
			} 
			Interaction::None => {
				lab_index.0 = 0;
			}
		}
	}
}

fn button_logbook (
	mut lab_query: Query<&mut LabSpriteIndex>,
	mut interaction_query: Query<(&LogbookButton, &Interaction), Changed<Interaction>>,
	mut state: ResMut<State<GameState>>,
) {
	let mut lab_index = lab_query.single_mut();
	for (_, interaction) in interaction_query.iter_mut() {
		match interaction {
			Interaction::Clicked => {
				state.push(GameState::Logbook).expect("Failed to change states");
			}
			Interaction::Hovered => {
				lab_index.0 = 1;
			} 
			Interaction::None => {
				lab_index.0 = 0;
			}
		}
	}
}

fn button_exit (
	mut lab_query: Query<&mut LabSpriteIndex>,
	mut interaction_query: Query<(&ExitButton, &Interaction), Changed<Interaction>>,
	mut state: ResMut<State<GameState>>,
) {
	let mut lab_index = lab_query.single_mut();
	for (_, interaction) in interaction_query.iter_mut() {
		match interaction {
			Interaction::Clicked => {
				state.pop().expect("Failed to change states");
			}
			Interaction::Hovered => {
				lab_index.0 = 3;
			} 
			Interaction::None => {
				lab_index.0 = 0;
			}
		}
	}
}
