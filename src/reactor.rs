use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use rand::{Rng, random};

use crate::{
	GameState,
	art::{spawn_spritesheet_sprite, SpriteSheets}, 
	enums::{MoleculeType, SpriteType, ReactionType},
	start_menu::despawn_screen, 
	reactor_ui::Reactor, 
	economy::{Economy, Power, Temperature, PRESSURE_SENSITIVITY}, 
	endgame::BasicCountdown
};

// ---------- PLUGINS ----------
pub struct ReactorPlugin;

impl Plugin for ReactorPlugin {
	fn build(&self, app: &mut App) {
		app
		.add_system_set(SystemSet::on_enter(GameState::Reactor) 
			.with_system(generate_thermometer)
			.with_system(generate_animation_timer)
		)
		.add_system_set(SystemSet::on_exit(GameState::Reactor)
			.with_system(despawn_screen::<Thermometer>)
		)
		.add_system_set(SystemSet::on_pause(GameState::Reactor)
			.with_system(despawn_screen::<Thermometer>)
		)
		.add_system_set(SystemSet::on_resume(GameState::Reactor) 
			.with_system(generate_thermometer)
		)
		.add_system_set(SystemSet::on_update(GameState::Reactor)
			.with_system(molecule_movement)
			.with_system(molecule_animation)
			.with_system(update_thermometer)
			.with_system(check_godmode)
		)
		;
	}
}

// ---------- RESOURCES ----------

const ANIMATION_SPEED: f32 = 0.125;

// ---------- COMPONENTS ----------
#[derive(Component, Inspectable)]
pub struct MoleculeList(pub Entity);

#[derive(Component, Inspectable)]
pub struct Cost{
	pub red_cost: f32,
	pub blue_cost: f32,
    pub orange_cost: f32,
    pub purple_cost: f32,
    pub gray_cost: f32,
}

impl Cost {
	pub fn get_cost(&self, molecule_type: MoleculeType) -> &f32 {
		match molecule_type {
			MoleculeType::Red => &self.red_cost,
			MoleculeType::Blue => &self.blue_cost,
			MoleculeType::Orange => &self.orange_cost,
            MoleculeType::Purple => &self.purple_cost,
            MoleculeType::Gray => &self.gray_cost,
		}
	}

	pub fn get_cost_mut(&mut self, molecule_type: MoleculeType) -> &mut f32 {
		match molecule_type {
			MoleculeType::Red => &mut self.red_cost,
			MoleculeType::Blue => &mut self.blue_cost,
			MoleculeType::Orange => &mut self.orange_cost,
            MoleculeType::Purple => &mut self.purple_cost,
            MoleculeType::Gray => &mut self.gray_cost,
		}
	}
}

#[derive(Component, Inspectable)]
pub struct GodMode(pub bool);

#[derive(Component, Inspectable)]
pub struct Unlocked {
	pub red_unlocked: bool,
	pub blue_unlocked: bool,
    pub orange_unlocked: bool,
    pub purple_unlocked: bool,
    pub gray_unlocked: bool,
}

impl Unlocked {  
    pub fn get_unlocked(&self, molecule_type: MoleculeType) -> &bool {
        match molecule_type {
			MoleculeType::Red => &self.red_unlocked,
			MoleculeType::Blue => &self.blue_unlocked,
			MoleculeType::Orange => &self.orange_unlocked,
            MoleculeType::Purple => &self.purple_unlocked,
            MoleculeType::Gray => &self.gray_unlocked,
		}
    }

	pub fn unlock(&mut self, molecule_type: MoleculeType) {
		match molecule_type {
			MoleculeType::Red => self.red_unlocked = true,
			MoleculeType::Blue => self.blue_unlocked = true,
			MoleculeType::Orange => self.orange_unlocked = true,
            MoleculeType::Purple => self.purple_unlocked = true,
            MoleculeType::Gray => self.gray_unlocked = true,
		}
	}
}

#[derive(Component, Inspectable)]
pub struct Molecule {
	pub variant: MoleculeType,
	pub mass: f32,
	pub reacted: bool,
}

#[derive(Component, Inspectable)]
pub struct Dimensions
{
	pub x_size: f32,
	pub y_size: f32
}

#[derive(Component, Inspectable)]
pub struct Velocity
{
	pub val: Vec2
}

#[derive(Component)]
struct Thermometer;

#[derive(Component)]
pub struct AnimationTimer;

// ---------- SYSTEMS ----------
fn generate_animation_timer (
	mut commands: Commands,
) {
	commands
		.spawn()
		.insert(BasicCountdown(Timer::from_seconds(ANIMATION_SPEED, true)))
		.insert(Name::new("Animation Refresh Rate Timer"))
		.insert(Reactor)
		.insert(AnimationTimer);
}

pub fn generate_molecule_list (
	mut commands: Commands,
) {
	let molecule_list = commands
		.spawn()
		.insert(Name::new("Molecule List"))
		.insert(Transform::default())
		.insert(GlobalTransform::default())
		.insert(ComputedVisibility::default())
		.insert(Visibility::visible())
		.id();

	commands
	.entity(molecule_list)
	.insert(MoleculeList(molecule_list));

	commands
	.spawn()
	.insert(Cost {
		red_cost: MoleculeType::Red.base_cost(),
		blue_cost: MoleculeType::Blue.base_cost(),
        orange_cost: MoleculeType::Orange.base_cost(),
        purple_cost: MoleculeType::Purple.base_cost(),
        gray_cost: MoleculeType::Gray.base_cost(),
	})
	.insert(Economy)
	.insert(Name::new("Molecule Costs"))
	;
}

// Check please
fn product_molecule (
    molecule_type: MoleculeType,
    mut commands: &mut Commands,
    spritesheet: &Res<SpriteSheets>,
    molecule_list_query: &mut Query<&MoleculeList>,
    location: Vec3,
) {
    let molecule_list = molecule_list_query.single_mut();

    let molecule = spawn_spritesheet_sprite(
		&mut commands,
		&spritesheet,
		SpriteType::Molecule(molecule_type),
		0,
		Color::WHITE,
		location,
		Vec2::splat(32.0)
	);

	let mass: f32 = molecule_type.mass();

	commands
	.entity(molecule)
	.insert(Name::new(molecule_type.name()))
	.insert(Molecule {
		variant: molecule_type,
		mass: molecule_type.mass(),
		reacted: false
	})
	.insert(Velocity {val: Vec2::new( // TODO: smarter value than just "4.0"
		4.0 * (2.0 * random::<f32>() - 1.0).signum() * ((2.0 * random::<f32>() - 1.0).abs() + random::<f32>() - 0.5).clamp(0.0, 1.0) / mass,
		4.0 * (2.0 * random::<f32>() - 1.0).signum() * ((2.0 * random::<f32>() - 1.0).abs() + random::<f32>() - 0.5).clamp(0.0, 1.0) / mass
	)})
	.insert(Dimensions {
		x_size: 32.0,
		y_size: 32.0
	});

	commands
	.entity(molecule_list.0)
	.push_children(&[molecule]);
}

pub fn generate_unlock_list (
	mut commands: Commands,
) {
	commands
    .spawn()
    .insert( Unlocked {
        red_unlocked: true,
        blue_unlocked: true,
        orange_unlocked: false,
        purple_unlocked: false,
        gray_unlocked: false,
    })
    .insert(Name::new("Unlocked"));

	commands
	.spawn()
	.insert(GodMode(false))
	.insert(Name::new("God Mode"));
}

fn check_godmode (
	keyboard: Res<Input<KeyCode>>,
	mut godmode_query: Query<&mut GodMode>,
	mut unlocked_query: Query<&mut Unlocked>,
) {
	let mut godmode = godmode_query.single_mut();
	let mut unlocked = unlocked_query.single_mut();
	if keyboard.just_pressed(KeyCode::G) {
		godmode.0 = !godmode.0;
	}
	if godmode.0 {
		unlocked.unlock(MoleculeType::Red);
		unlocked.unlock(MoleculeType::Blue);
		unlocked.unlock(MoleculeType::Orange);
		unlocked.unlock(MoleculeType::Purple);
		unlocked.unlock(MoleculeType::Gray);
	}

}

pub fn add_molecule (
	molecule_type: MoleculeType,
	mut commands: &mut Commands,
	spritesheet: &Res<SpriteSheets>,
	molecule_list_query: &mut Query<&MoleculeList>,
	reactor_query: &Query<(&Reactor, &Dimensions, &Transform)>
) {
	let (_reactor, dimensions, transform) = reactor_query.single();

	let molecule_list = molecule_list_query.single_mut();

	// Make this global resource?
	let mut rng = rand::thread_rng();
	let molecule = spawn_spritesheet_sprite(
		&mut commands,
		&spritesheet,
		SpriteType::Molecule(molecule_type),
		rng.gen_range(0..8),
		Color::WHITE,
		//Color::rgb(random::<f32>(), random::<f32>(), random::<f32>()),
		// Find some way to pass Reactor to spawn molecule inside and bound movement within
		// Pass reactor index to function, use reactor index to find reactor size from transform.scale
		Vec3::new(
		(dimensions.x_size * random::<f32>() - (dimensions.x_size / 2.0)) + transform.translation.x, 
		(dimensions.y_size * random::<f32>() - (dimensions.y_size / 2.0)) + transform.translation.y, 
		900.0),
		Vec2::splat(32.0)
	);

	let mass: f32 = molecule_type.mass();

	commands
	.entity(molecule)
	.insert(Name::new(molecule_type.name()))
	.insert(Molecule {
		variant: molecule_type,
		mass: molecule_type.mass(),
		reacted: false
	})
	.insert(Velocity {val: Vec2::new( // TODO: smarter value than just "4.0"
		4.0 * (2.0 * random::<f32>() - 1.0).signum() * ((2.0 * random::<f32>() - 1.0).abs() + random::<f32>() - 0.5).clamp(0.0, 1.0) / mass,
		4.0 * (2.0 * random::<f32>() - 1.0).signum() * ((2.0 * random::<f32>() - 1.0).abs() + random::<f32>() - 0.5).clamp(0.0, 1.0) / mass
	)})
	.insert(Dimensions {
		x_size: 32.0,
		y_size: 32.0
	});

	commands
	.entity(molecule_list.0)
	.push_children(&[molecule]);
}

pub fn add_control_rod (
	mut commands: Commands,
	entities: Query<Entity, With<Molecule>>,
) {
	for entity in &entities {
		commands.entity(entity).despawn_recursive();
	}
}

fn molecule_movement (
    mut commands: Commands,
    spritesheet: Res<SpriteSheets>,
	mut reactor_query: Query<(&Reactor, &Dimensions, &Transform), Without<Molecule>>,
	mut molecule_query: Query<(Entity, &mut Molecule, &mut Velocity, &Dimensions, &mut Transform)>,
	mut molecule_list_query: Query<&MoleculeList>,
	mut economy_query: Query<(&Economy, &mut Power, &mut Temperature)>,
	mut unlocked_query: Query<&mut Unlocked>,
) {
	let (_economy, mut power, mut pressure) = economy_query.single_mut();
	let mut unlocked = unlocked_query.single_mut();

	// Remove single mut when adding more reactors
	let (_reactor, reactor_size, reactor_transform) = reactor_query.single_mut();

	let collision_radius = 32.0;

	let mut iter = molecule_query.iter_combinations_mut();
	while let Some([
			(entity_a, mut molecule_a, mut velocity_a, _, mut transform_a),
			(entity_b, mut molecule_b, mut velocity_b, _, mut transform_b)
		]) = iter.fetch_next() {
		// To prevent particles reacting multiple times at once somehow
		if molecule_a.reacted || molecule_b.reacted {
			continue;
		}
		let offset = transform_a.translation - transform_b.translation;
		let offset = Vec2::new(offset.x, offset.y);
		if offset.length_squared() <= collision_radius * collision_radius {
			if let Some(reaction) = MoleculeType::can_react(molecule_a.variant, molecule_b.variant) {
				let mut reaction_success = false;
				match reaction.reaction_type {
					ReactionType::RedBlue | ReactionType::GrayPurple | ReactionType::GrayRed | ReactionType::GrayPurple => {
						// If temp > xyz or something
						for product in &reaction.products {
							// Change location to midpoint between molecules?
							product_molecule(*product, &mut commands, &spritesheet, &mut molecule_list_query, transform_a.translation);
							reaction_success = true;}
					}
					ReactionType::BlueBlue | ReactionType::OrangeOrange | ReactionType::GrayOrange => {
						for product in &reaction.products {
							let loc_rng = Vec3::new(random::<f32>(), random::<f32>(),random::<f32>());
							// Change location to midpoint between molecules?
							product_molecule(*product, &mut commands, &spritesheet, &mut molecule_list_query, transform_a.translation + loc_rng);
							reaction_success = true;}
					}
				}

				if reaction_success {
					for product in reaction.products {
						if !unlocked.get_unlocked(product) {
							unlocked.unlock(product);
						}
					}
					power.current_power = (power.current_power + reaction.power_generated).clamp(0.0, 999000.0);
					//pressure.current_pressure = (pressure.current_pressure + reaction.temp_generated).clamp(0.0, MAX_PRESSURE);
					// Molecules that react are despawned anyway
					molecule_a.reacted = true;
					molecule_b.reacted = true;
					commands.entity(entity_a).despawn_recursive();
					commands.entity(entity_b).despawn_recursive();
				}
				// TODO: non elastic-bounce interactions
				// Should despawn these two particles and spawn a new one with the reaction.product type
				// Consider adding a "internal energy" to the resulting molecule, and then have it decay randomly if it's too much
				// resulting speed should be vel = (vel_a * mass_a + vel_b * mass_b) / mass_result
				// mass of resulting particle should be the sum of masses probably
			} else {
				// Boing happens here
				let relative_velocity = velocity_a.val - velocity_b.val;
				let dp_kinda_thing = offset * Vec2::dot(relative_velocity, offset) / ((offset.length_squared()) * (molecule_a.mass + molecule_b.mass));

				velocity_a.val -= 2.0 * molecule_b.mass * dp_kinda_thing;
				velocity_b.val += 2.0 * molecule_a.mass * dp_kinda_thing;

				let push = (offset.normalize() * 1.01 * collision_radius - offset).extend(0.0);
				transform_a.translation += push;
				transform_b.translation -= push;
			}
		}
	}

	let mut delta_momentum = 0.0;

	for (_entity, molecule, mut velocity, molecule_size, mut transform) in molecule_query.iter_mut() {
		//this line is no longer needed cause particles that reacted should just be deleted next time
		// molecule.reacted = false;
		// If molecule is on collision course then reverse path with some randomness
		if false {
			velocity.val.x = -velocity.val.x;
			transform.translation.x = transform.translation.x + velocity.val.x.signum() * ((velocity.val.y).abs() + random::<f32>() - 0.5).clamp(0.0, 8.0);
			velocity.val.y = -velocity.val.y;
			transform.translation.y = transform.translation.y + velocity.val.y.signum() * ((velocity.val.y).abs() + random::<f32>() - 0.5).clamp(0.0, 8.0);
		}
		else {
			let x_target = transform.translation.x + velocity.val.x;
			// If molecule won't hit off of the edges
			if (x_target - reactor_transform.translation.x).abs() <= (reactor_size.x_size  - molecule_size.x_size) / 2.0 {
				transform.translation.x = x_target;
			}
			// If molecule will hit off of the edges
			else {
				velocity.val.x = -velocity.val.x;
				delta_momentum += (2.0 * velocity.val.x * molecule.mass).abs();
			}

			let y_target = transform.translation.y + velocity.val.y;
			// If molecule won't hit off of the top or bottom
			if (y_target - reactor_transform.translation.y).abs() <= (reactor_size.y_size - molecule_size.y_size) / 2.0 {
				transform.translation.y = y_target;
			}
			// If molecule will hit off of the top or bottom
			else {
				velocity.val.y = -velocity.val.y;
				delta_momentum += (2.0 * velocity.val.y * molecule.mass).abs();
			}
		}

		// How quickly the pressure should change
		// 1 = updates completely every frame
		// 0 = doesnt do anything
		// /60 is to try to compensate for framerate
		let lerp_factor = PRESSURE_SENSITIVITY / 60.0;

		pressure.current_pressure *= 1.0 - lerp_factor;
		// The * 60 is because of the short timespan of the collision = large force
		pressure.current_pressure += 60.0 * delta_momentum * lerp_factor;

		// Clamp molecules within reactor so they don't get bounced outside by collisions
		transform.translation.x = transform.translation.x.clamp((-(reactor_size.x_size - molecule_size.x_size) / 2.0) + reactor_transform.translation.x, ((reactor_size.x_size - molecule_size.x_size) / 2.0) + reactor_transform.translation.x);
		transform.translation.y = transform.translation.y.clamp((-(reactor_size.y_size - molecule_size.y_size) / 2.0) + reactor_transform.translation.y, ((reactor_size.y_size - molecule_size.y_size) / 2.0) + reactor_transform.translation.y );
	}
}

fn molecule_animation (
	time: Res<Time>,
	mut molecule_query: Query<(&Molecule, &mut TextureAtlasSprite)>,
	mut animation_timer_query: Query<(&AnimationTimer, &mut BasicCountdown)>,
) {
	let (_animation_timer, mut countdown) = animation_timer_query.single_mut();
	if countdown.0.tick(time.delta()).just_finished() {
		for (molecule, mut sprite) in molecule_query.iter_mut() {
			sprite.index += 1;
			sprite.index %= molecule.variant.animation_frames();
		}
	}
}

fn generate_thermometer (
	mut commands: Commands,
	spritesheet: Res<SpriteSheets>,
) {
	let thermometer = spawn_spritesheet_sprite(
		&mut commands,
		&spritesheet,
		SpriteType::Thermometer,
		0,
		Color::WHITE,
		Vec3::new(
		645.0, 
		135.0, 
		905.0),
		Vec2::new(120.0, 440.0)
	);

	commands
		.entity(thermometer)
		.insert(Name::new("Thermometer"))
		.insert(BasicCountdown(Timer::from_seconds(2.0, true)))
		.insert(Thermometer)
		;
}

fn update_thermometer (
	time: Res<Time>,
	//molecule_query: Query<(&Molecule, &Velocity)>,
	mut state: ResMut<State<GameState>>,
	mut thermometer_query: Query<(&Thermometer, &mut BasicCountdown, &mut TextureAtlasSprite)>,
	mut economy_query: Query<(&Economy, &mut Temperature)>,
	godmode_query: Query<&GodMode>
) {
	let godmode = godmode_query.single();
	let (_thermometer, mut cooling_countdown, mut thermometer_sprite) = thermometer_query.single_mut();
	let (_economy, pressure) = economy_query.single_mut();

	if !godmode.0 {
		if cooling_countdown.0.tick(time.delta()).just_finished() {
			// If current pressure above max then cut to boomScreen
			if pressure.current_pressure >= pressure.max_pressure {
				state.push(GameState::BoomScreen).expect("Failed to change states");
			}
			// Otherwise subtract passive cooling
			//temperature.current_temperature = (temperature.current_temperature - 3.0).clamp(0.0, temperature.max_temperature);
		}
	}

	// Update thermometer sprite based on current pressure
	thermometer_sprite.index = ((pressure.current_pressure / pressure.max_pressure * 6.0).round() as usize).clamp(0, 6);
}
