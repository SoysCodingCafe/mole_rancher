use bevy::prelude::*;

use crate::{INVISIBLE, GameState, start_menu::despawn_screen, art::{SpriteSheets, spawn_spritesheet_sprite}, enums::SpriteType};

// ---------- PLUGINS ----------
pub struct LogbookPlugin;

impl Plugin for LogbookPlugin {
	fn build(&self, app: &mut App) {
        app
        .add_system_set(SystemSet::on_enter(GameState::Logbook)
            .with_system(load_logbook)
		)
        .add_system_set(SystemSet::on_resume(GameState::Logbook)
            .with_system(load_logbook)
        )
		.add_system_set(SystemSet::on_update(GameState::Logbook)
			.with_system(button_exit_logbook)
		)
		.add_system_set(SystemSet::on_pause(GameState::Logbook)
            .with_system(despawn_screen::<LogbookSprites>)
            .with_system(despawn_screen::<LogbookUi>)
		)
		.add_system_set(SystemSet::on_exit(GameState::Logbook)
            .with_system(despawn_screen::<LogbookSprites>)
            .with_system(despawn_screen::<LogbookUi>)
		)
        ;
    }
}

// ---------- RESOURCES ----------


// ---------- COMPONENTS ----------
#[derive(Component)]
struct LogbookSprites;

#[derive(Component)]
struct LogbookUi;

#[derive(Component)]
struct LogbookExitButton;

#[derive(Component)]
struct LogbookExitButtonSprite;

fn load_logbook (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
	spritesheet: Res<SpriteSheets>,
) {
	let variant = SpriteType::LogbookButton;
	let location = Vec3::new(668.0, -342.0, 300.0);
	let sprite = spawn_spritesheet_sprite(
		&mut commands,
		&spritesheet,
		variant,
		0,
		Color::WHITE,
		location,
		Vec2::splat(128.0)
	);
	
	commands
	.entity(sprite)
	.insert(LogbookSprites)
	.insert(LogbookExitButtonSprite)
	.insert(Name::new("Logbook Sprite"));

    commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("logbook.png"),
			transform: Transform::from_scale(Vec3::new(4.0, 4.0, 4.0)).with_translation(Vec3::new(0.0, 0.0, 300.0)),
			..default()
		})
		.insert(Name::new("Logbook Sprite"))
        .insert(LogbookSprites)
        ;

	let size = 128.0;
	commands
		.spawn_bundle(NodeBundle {
			style: Style {
				position_type: PositionType::Absolute,
				position: UiRect {
					// Screen width, sprite width, x translation
					left: Val::Px(1600.0 / 2.0 - size / 2.0 + location.x),
					// Screen height, sprite height, y translation
					bottom: Val::Px(900.0 / 2.0 - size / 2.0 + location.y), 
					..default()
				},
				// Sprite width and height
				size: Size::new(Val::Px(size), Val::Px(size)),
				..default()
			},
			color: INVISIBLE.into(),
			..default()
		})
		.insert(Name::new("Logbook Exit Node"))
		.insert(LogbookUi)
		.with_children(|parent| {

			// Reactor Button
			parent
				.spawn_bundle(ButtonBundle {
					style: Style {
						// Sprite width and height
						size: Size::new(Val::Px(size), Val::Px(size)),
						..default()
					},
					color: INVISIBLE.into(),
					..default()
				})
				.insert(LogbookExitButton);
	});
}

fn button_exit_logbook (
	mut logbook_back_button_sprite_query: Query<(&LogbookExitButtonSprite, &mut TextureAtlasSprite)>,
	mut interaction_query: Query<(&LogbookExitButton, &Interaction), Changed<Interaction>>,
	mut state: ResMut<State<GameState>>,
) {
	let (_, mut sprite) = logbook_back_button_sprite_query.single_mut();
	for (_, interaction) in interaction_query.iter_mut() {
		match interaction {
			Interaction::Clicked => {
				state.pop().expect("Failed to change states");
			}
			Interaction::Hovered => {
				sprite.index = 0;
			} 
			Interaction::None => {
				sprite.index = 1;
			}
		}
	}
}