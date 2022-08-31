use bevy::prelude::*;

// ---------- PLUGINS ----------
pub struct VisualPlugin;

impl Plugin for VisualPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system_to_stage(StartupStage::PreStartup, setup_spritesheet);
	}
}

// ---------- RESOURCES ----------
// Assets
pub struct SpriteSheet(pub Handle<TextureAtlas>);

// ---------- SYSTEMS ----------
/*
fn setup_spritesheet(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
	// Load spritesheet into texture atlas
	let texture_handle = asset_server.load("spritesheet.png");
	let texture_atlas = TextureAtlas::from_grid_with_padding(
		texture_handle,
		Vec2::splat(9.0),
		16,
		16,
		Vec2::splat(2.0),
		Vec2::splat(0.0)
	);
	let texture_atlas_handle = texture_atlases.add(texture_atlas);

	// Passes texture atlas handle to SpriteSheet resource so it can be accessed in other systems
	commands.insert_resource(SpriteSheet(texture_atlas_handle));
}*/

fn setup_spritesheet(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
	// Load spritesheet into texture atlas
	let texture_handle = asset_server.load("molecule_blueboi.png");
	let texture_atlas = TextureAtlas::from_grid_with_padding(
		texture_handle,
		Vec2::splat(32.0),
		4,
		4,
		Vec2::splat(0.0),
		Vec2::splat(0.0)
	);
	let texture_atlas_handle = texture_atlases.add(texture_atlas);

	// Passes texture atlas handle to SpriteSheet resource so it can be accessed in other systems
	commands.insert_resource(SpriteSheet(texture_atlas_handle));
}

pub fn spawn_spritesheet_sprite(
	commands: &mut Commands,
	spritesheet: &SpriteSheet,
	index: usize,
	color: Color,
	translation: Vec3,
	size: Vec2,
) -> Entity {
	let mut sprite = TextureAtlasSprite::new(index);
	sprite.color = color;
	sprite.custom_size = Some(size);

	commands.spawn_bundle(SpriteSheetBundle {
		sprite: sprite,
		texture_atlas: spritesheet.0.clone(),
		transform: Transform {
			translation: translation,
			..Default::default()
		},
		..Default::default()
	}).id() // id() gives back the entity after creation
}
