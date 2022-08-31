use bevy::prelude::*;
use rand::{Rng, random};

use crate::{visual::{spawn_spritesheet_sprite, SpriteSheet}, molecule::{MoleculeParent, Molecule, Velocity, Dimensions}, GameState};

// ---------- PLUGINS ----------
pub struct UiPlugin;

impl Plugin for UiPlugin {
	fn build(&self, app: &mut App) {
		app
		.add_system_set(
			SystemSet::on_enter(GameState::Game)
				.with_system(setup_button)
		)
		.add_system_set(
			SystemSet::on_update(GameState::Game)
				.with_system(button_system)
		)
		;
	}
}

// ---------- RESOURCES ----------
const IDLE_COLOR: Color = Color::rgb(0.6, 0.6, 0.6);
const HOVER_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const CLICK_COLOR: Color = Color::rgb(0.1, 0.8, 0.8);

// ---------- SYSTEMS ----------
fn setup_button(
	mut commands: Commands,
	asset_server: Res<AssetServer>
) {
	commands
	.spawn_bundle(ButtonBundle {
		style: Style {
			size: Size::new(Val::Px(150.0), Val::Px(65.0)),
			// Center button
			margin: UiRect {
				// Distance between left edge and button
				bottom: bevy::prelude::Val::Px(800.0),
				left: bevy::prelude::Val::Px(800.0 - 65.0),
				..default()
			},
			// Horizontally center child text
			justify_content: JustifyContent::Center,
			// Vertically center child text
			align_items: AlignItems::Center,
			..default()
		},
		color: IDLE_COLOR.into(),
		..default()
	})
	.with_children(|parent| {
		parent.spawn_bundle(TextBundle::from_section(
			"Button",
			TextStyle {
				font: asset_server.load("fonts/FiraSans-Bold.ttf"),
				font_size: 40.0,
				color: Color::rgb(0.1, 0.1, 0.1),
			},
		));
	});
}

fn button_system(
	mut interaction_query: Query<
		(&Interaction, &mut UiColor, &Children),
		(Changed<Interaction>, With<Button>),
	>,
	mut text_query: Query<&mut Text>,
	mut commands: Commands,
	spritesheet: Res<SpriteSheet>,
	molecule_parent: Res<MoleculeParent>
) {
	for (interaction, mut color, children) in &mut interaction_query {
		let mut text = text_query.get_mut(children[0]).unwrap();
		match *interaction {
			Interaction::Clicked => {
				text.sections[0].value = "Press".to_string();
				*color = CLICK_COLOR.into();

				// Duplicated since original triggered on spacebar press
				// ---------- COPY OF GENERATE MOLECULE ---------
				// Make this global resource?
				let mut rng = rand::thread_rng();
				let molecule = spawn_spritesheet_sprite(
					&mut commands,
					&spritesheet,
					// [0, 16)
					rng.gen_range(0..16),
					Color::rgb(random::<f32>(), random::<f32>(), random::<f32>()),
					// Find some way to pass Reactor to spawn molecule inside and bound movement within
					Vec3::new(0.0, 0.0, 900.0),
					// Find some way to connect value to Size component
					Vec2::splat(32.0)
				);

				commands
				.entity(molecule)
				.insert(Name::new("Molecule"))
				.insert(Molecule {
					cost: 60.0
				})
				.insert(Velocity {
					x_vel: 16.0 * random::<f32>() - 8.0,
					y_vel: 16.0 * random::<f32>() - 8.0,
					z_vel: 0.2 * random::<f32>() - 0.1,
				})
				.insert(Dimensions {
					x_size: 16.0 + 4.0 * random::<f32>(),
					y_size: 16.0 + 4.0 * random::<f32>()
				});

				commands
				.entity(molecule_parent.list[0])
				.push_children(&[molecule]);
			}
			// ---------- END COPY OF GENERATE MOLECULE ---------

			Interaction::Hovered => {
				text.sections[0].value = "Hover".to_string();
				*color = HOVER_COLOR.into();
			}
			Interaction::None => {
				text.sections[0].value = "Button".to_string();
				*color = IDLE_COLOR.into();
			}
		}
	}
}
