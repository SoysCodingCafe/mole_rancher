use bevy::prelude::*;

use crate::enums::{SpriteType, MoleculeType};

// ---------- PLUGINS ----------
pub struct ArtPlugin;

impl Plugin for ArtPlugin {
	fn build(&self, app: &mut App) {
		app
		.add_startup_system_to_stage(StartupStage::PreStartup, load_spritesheets)
		;
	}
}

// ---------- RESOURCES ----------
// Assets

// Individual struct for each handle (make enum so handled by single function?)
//pub struct RedSprites(pub Handle<TextureAtlas>);
//pub struct BlueSprites(pub Handle<TextureAtlas>);

pub struct SpriteSheets {
	pub red_molecule: Handle<TextureAtlas>,
	pub blue_molecule: Handle<TextureAtlas>,
	pub orange_molecule: Handle<TextureAtlas>,
    pub purple_molecule: Handle<TextureAtlas>,
    pub gray_molecule: Handle<TextureAtlas>,
	pub lab: Handle<TextureAtlas>,
	pub thermometer: Handle<TextureAtlas>,
    pub logbook_button: Handle<TextureAtlas>,
}

impl SpriteSheets {
	pub fn get_atlas(&self, sprite: SpriteType) -> Handle<TextureAtlas> {
		match sprite {
			SpriteType::Lab => self.lab.clone(),
			SpriteType::Thermometer => self.thermometer.clone(),
            SpriteType::LogbookButton => self.logbook_button.clone(),
			SpriteType::Molecule(molecule_type) => self.get_molecule_atlas(molecule_type)
		}
	}

	pub fn get_molecule_atlas(&self, molecule_type: MoleculeType) -> Handle<TextureAtlas> {
		match molecule_type {
			MoleculeType::Red => self.red_molecule.clone(),
			MoleculeType::Blue => self.blue_molecule.clone(),
			MoleculeType::Orange => self.orange_molecule.clone(),
            MoleculeType::Purple => self.purple_molecule.clone(),
            MoleculeType::Gray => self.gray_molecule.clone(),
		}
	}
}

// ---------- SYSTEMS ----------
fn load_spritesheets(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
	// TODO: Create a function to do this so I don't have to copy and paste 5 times
	let red_handle = asset_server.load("molecules/molecule_redboi.png");
	let red_atlas = TextureAtlas::from_grid_with_padding(
		red_handle,
		Vec2::splat(32.0),
		// Create component to store spritesheet rows and columns so that
		// maximum index can be calculated and stored for each spritesheet
		4,
		4,
		Vec2::splat(0.0),
		Vec2::splat(0.0)
	);

	let blue_handle = asset_server.load("molecules/molecule_blueboi.png");
	let blue_atlas = TextureAtlas::from_grid_with_padding(
		blue_handle,
		Vec2::splat(32.0),
		4,
		4,
		Vec2::splat(0.0),
		Vec2::splat(0.0)
	);

	let orange_handle = asset_server.load("molecules/molecule_orangeboi.png");
	let orange_atlas = TextureAtlas::from_grid_with_padding(
		orange_handle,
		Vec2::splat(32.0),
		4,
		4,
		Vec2::splat(0.0),
		Vec2::splat(0.0)
	);

    let purple_handle = asset_server.load("molecules/molecule_purpleboi.png");
	let purple_atlas = TextureAtlas::from_grid_with_padding(
		purple_handle,
		Vec2::splat(32.0),
		4,
		4,
		Vec2::splat(0.0),
		Vec2::splat(0.0)
	);

    let gray_handle = asset_server.load("molecules/molecule_grayboi.png");
	let gray_atlas = TextureAtlas::from_grid_with_padding(
		gray_handle,
		Vec2::splat(32.0),
		4,
		4,
		Vec2::splat(0.0),
		Vec2::splat(0.0)
	);

	let lab_handle = asset_server.load("lab.png");
	let lab_atlas = TextureAtlas::from_grid_with_padding(
		lab_handle,
		Vec2::new(400.0, 225.0),
		// Create component to store spritesheet rows and columns so that
		// maximum index can be calculated and stored for each spritesheet
		2,
		2,
		Vec2::splat(0.0),
		Vec2::splat(0.0)
	);

	let thermometer_handle = asset_server.load("thermometer.png");
	let thermometer_atlas = TextureAtlas::from_grid_with_padding(
		thermometer_handle,
		Vec2::new(30.0, 110.0),
		// Create component to store spritesheet rows and columns so that
		// maximum index can be calculated and stored for each spritesheet
		4,
		2,
		Vec2::splat(0.0),
		Vec2::splat(0.0)
	);

    let logbook_button_handle = asset_server.load("logbook_button.png");
	let logbook_button_atlas = TextureAtlas::from_grid_with_padding(
		logbook_button_handle,
		Vec2::new(32.0, 32.0),
		// Create component to store spritesheet rows and columns so that
		// maximum index can be calculated and stored for each spritesheet
		2,
		1,
		Vec2::splat(0.0),
		Vec2::splat(0.0)
	);

	let red_atlas_handle = texture_atlases.add(red_atlas);
	let blue_atlas_handle = texture_atlases.add(blue_atlas);
	let orange_atlas_handle = texture_atlases.add(orange_atlas);
    let purple_atlas_handle = texture_atlases.add(purple_atlas);
	let gray_atlas_handle = texture_atlases.add(gray_atlas);
	let lab_atlas_handle = texture_atlases.add(lab_atlas);
	let thermometer_atlas_handle = texture_atlases.add(thermometer_atlas);
    let logbook_button_atlas_handle = texture_atlases.add(logbook_button_atlas);

	// Passes texture atlas handles to resources so they can be accessed in other systems
	//commands.insert_resource(RedSprites(red_atlas_handle));
	//commands.insert_resource(BlueSprites(blue_atlas_handle));
	commands.insert_resource(SpriteSheets {
		red_molecule: red_atlas_handle,
		blue_molecule: blue_atlas_handle,
		orange_molecule: orange_atlas_handle,
        purple_molecule: purple_atlas_handle,
        gray_molecule: gray_atlas_handle,
		lab: lab_atlas_handle,
		thermometer: thermometer_atlas_handle,
        logbook_button: logbook_button_atlas_handle,
	});
}

pub fn spawn_spritesheet_sprite(
	commands: &mut Commands,
	spritesheet: &SpriteSheets,
	sprite_type: SpriteType,
	index: usize,
	color: Color,
	translation: Vec3,
	size: Vec2,
) -> Entity {
	let mut sprite = TextureAtlasSprite::new(index);
	sprite.color = color;
	sprite.custom_size = Some(size);

	let atlas = spritesheet.get_atlas(sprite_type);

	commands.spawn_bundle(SpriteSheetBundle {
		sprite: sprite,
		texture_atlas: atlas,
		transform: Transform {
			translation: translation,
			..Default::default()
		},
		..Default::default()
	}).id() // id() gives back the entity after creation
}
