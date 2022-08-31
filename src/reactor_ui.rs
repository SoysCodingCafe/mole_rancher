use bevy::{prelude::*, ui::FocusPolicy};

use crate::{
	GameState, 
	start_menu::{despawn_screen, UiAssets}, 
	reactor::{Dimensions, MoleculeList, add_molecule, add_control_rod, Molecule, Cost, AnimationTimer, Unlocked, GodMode},
	art::{SpriteSheets, spawn_spritesheet_sprite}, economy::{Economy, Power, Clock},
	INVISIBLE,
	enums::{MoleculeType, SpriteType}, endgame::BasicCountdown
};

// ---------- PLUGINS ----------
pub struct ReactorUiPlugin;

impl Plugin for ReactorUiPlugin {
	fn build(&self, app: &mut App) {
		app
		.add_system_set(SystemSet::on_enter(GameState::Reactor) 
			.with_system(generate_monitor)
			.with_system(generate_reactor)
			.with_system(generate_text)
			.with_system(generate_cost_buttons)
			.with_system(generate_cost_button_sprites)
			.with_system(generate_button_monitor)
			.with_system(generate_button_standby)
		)
		.add_system_set(SystemSet::on_exit(GameState::Reactor)
			.with_system(despawn_screen::<ReactorSprites>)
			.with_system(despawn_screen::<Reactor>)
			.with_system(despawn_screen::<ReactorUi>)
		)
		.add_system_set(SystemSet::on_pause(GameState::Reactor)
			.with_system(despawn_screen::<ReactorSprites>)
			.with_system(despawn_screen::<Reactor>)
			.with_system(despawn_screen::<ReactorUi>)
		)
		.add_system_set(SystemSet::on_resume(GameState::Reactor)
		)
		.add_system_set(SystemSet::on_update(GameState::Reactor)
			.with_system(calculate_cost_text)
			.with_system(update_reactor_size)
			.with_system(update_economy_text)
			.with_system(update_button_cost_sprites)
			.with_system(button_cost)
			//.with_system(button_control_rod)
			.with_system(button_standby)
		)
		;
	}
}

// ---------- RESOURCES ----------
const COST_SCALING: f32 = 1.0;

// ---------- COMPONENTS ----------
#[derive(Component)]
struct ReactorSprites;

#[derive(Component)]
pub struct Reactor;

#[derive(Component)]
pub struct ReactorUi;

#[derive(Component)]
struct CostButtonSprites;

#[derive(Component)]
struct TimeText;

#[derive(Component)]
struct PowerText;

#[derive(Component)]
struct TargetText;

#[derive(Component)]
struct ButtonHovered {
	hovered_button: Option<ButtonType>
}

#[derive(Component)]
enum TextType {
	RedboiText,
	BlueboiText,
	OrangeboiText,
	PurpleboiText,
    GrayboiText,
}

#[derive(Component, Debug, PartialEq, Clone, Copy)]
enum ButtonType {
	MoleculeButton(MoleculeType),
}

#[derive(Component)]
struct CostButton;

#[derive(Component)]
struct StandbyButton;

// ---------- SYSTEMS ----------
fn generate_button_monitor (
	mut commands: Commands
) {
	commands
		.spawn()
		.insert(ButtonHovered {hovered_button: None})
		.insert(Reactor)
		.insert(Name::new("Button Monitor"));
}

fn generate_button_standby (
	mut commands: Commands,
) {
	commands
		.spawn_bundle(NodeBundle {
			style: Style {
				position_type: PositionType::Absolute,
				position: UiRect {
					// Screen width, sprite width, x translation
					left: Val::Px(1600.0 / 2.0 - 100.0 / 2.0 + 632.5),
					// Screen height, sprite height, y translation
					bottom: Val::Px(900.0 / 2.0 - 100.0 / 2.0 - 363.8), 
					..default()
				},
				// Sprite width and height
				size: Size::new(Val::Px(100.0), Val::Px(100.0)),
				..default()
			},
			color: INVISIBLE.into(),
			..default()
		})
		.insert(Name::new("Standby Node"))
		.insert(ReactorUi)
		.with_children(|parent| {

			// Standby Button
			parent
				.spawn_bundle(ButtonBundle {
					style: Style {
						size: Size::new(Val::Px(100.0), Val::Px(100.0)),
						..default()
					},
					color: INVISIBLE.into(),
					..default()
				})
				.insert(StandbyButton);
		});
}

fn generate_monitor (
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("monitor.png"),
			transform: Transform::from_scale(Vec3::new(4.0, 4.0, 0.0))
				.with_translation(Vec3::new(0.0, 0.0, 100.0)),
			..default()
	})
	.insert(Name::new("Monitor"))
	.insert(ReactorSprites)
	;

	// Test square for adjusting UI positioning
	/*commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("test_square.png"),
			transform: Transform::from_scale(Vec3::splat(1.0)).with_translation(Vec3::new(0.0, 0.0, 999.0)),
			..default()
		})
		.insert(Name::new("Test Square"));*/
}

fn generate_reactor (
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	let width = 840.0; 
	let height = 460.0;
	commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("test_square.png"),
			visibility: Visibility { is_visible: false },
			// 100.0 is sprite width and height (check update_reactor_size)
			transform: Transform::from_scale(Vec3::new(width / 100.0 , height / 100.0, 0.0))
				.with_translation(Vec3::new(120.0, 136.0, 300.0)),
			..default()
	})    
	.insert(Name::new("Reactor"))
	.insert(Reactor)
	.insert(Dimensions {
		x_size: width,
		y_size: height,
	})
	.insert(ReactorSprites)
	;
}

fn generate_text (
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	let font = asset_server.load("fonts/PixelSplitterBold.ttf");

	commands
	.spawn_bundle(NodeBundle {
		style: Style {
			position_type: PositionType::Absolute,
			position: UiRect {
				// Screen width, sprite width, x translation
				left: Val::Px(1600.0 / 2.0 - 150.0 / 2.0 - 420.0),
				// Screen height, sprite height, y translation
				bottom: Val::Px(900.0 / 2.0 - 70.0 / 2.0 - 160.0), 
				..default()
			},
			// Sprite width and height
			size: Size::new(Val::Px(180.0), Val::Px(100.0)),
			..default()
		},
		color: INVISIBLE.into(),
		..default()
	})
	.insert(Name::new("Time Node"))
	.insert(ReactorUi)
	.with_children(|parent| {
		parent
			.spawn_bundle(TextBundle::from_section(
				"10:00",
				TextStyle {
					font: font.clone(),
					font_size: 64.0,
					color: Color::rgb(0.0, 0.3, 0.0),
				},
			),
		)
		.insert(TimeText)
		;
	});

	commands
	.spawn_bundle(NodeBundle {
		style: Style {
			position_type: PositionType::Absolute,
			position: UiRect {
				// Screen width, sprite width, x translation
				left: Val::Px(1600.0 / 2.0 - 150.0 / 2.0 + 70.0),
				// Screen height, sprite height, y translation
				bottom: Val::Px(900.0 / 2.0 - 70.0 / 2.0 - 160.0), 
				..default()
			},
			// Sprite width and height
			size: Size::new(Val::Px(150.0), Val::Px(70.0)),
			..default()
		},
		color: INVISIBLE.into(),
		..default()
	})
	.insert(Name::new("Power Node"))
	.insert(ReactorUi)
	.with_children(|parent| {
		parent
			.spawn_bundle(TextBundle::from_section(
				"100",
				TextStyle {
					font: font.clone(),
					font_size: 64.0,
					color: Color::rgb(0.0, 0.3, 0.0),
				},
			),
		)
		.insert(PowerText)
		;
	});

	commands
	.spawn_bundle(NodeBundle {
		style: Style {
			position_type: PositionType::Absolute,
			position: UiRect {
				// Screen width, sprite width, x translation
				left: Val::Px(1600.0 / 2.0 - 150.0 / 2.0 + 615.0),
				// Screen height, sprite height, y translation
				bottom: Val::Px(900.0 / 2.0 - 70.0 / 2.0 - 160.0), 
				..default()
			},
			// Sprite width and height
			size: Size::new(Val::Px(150.0), Val::Px(70.0)),
			..default()
		},
		color: INVISIBLE.into(),
		..default()
	})
	.insert(Name::new("Target Node"))
	.insert(ReactorUi)
	.with_children(|parent| {
		parent
			.spawn_bundle(TextBundle::from_section(
				"2k",
				TextStyle {
					font: font.clone(),
					font_size: 64.0,
					color: Color::rgb(0.0, 0.3, 0.0),
				},
			),
		)
		.insert(TargetText)
		;
	});
}

pub fn generate_cost_buttons (
	mut commands: Commands,
	ui_assets: Res<UiAssets>,
) {

	let redboi_button = generate_cost_button(&mut commands, &ui_assets, 320.0, ButtonType::MoleculeButton(MoleculeType::Red), TextType::RedboiText);
	let blueboi_button = generate_cost_button(&mut commands, &ui_assets, 228.0, ButtonType::MoleculeButton(MoleculeType::Blue), TextType::BlueboiText);
	let orangeboi_button = generate_cost_button(&mut commands, &ui_assets, 136.0, ButtonType::MoleculeButton(MoleculeType::Orange), TextType::OrangeboiText);
	let purpleboi_button = generate_cost_button(&mut commands, &ui_assets, 44.0, ButtonType::MoleculeButton(MoleculeType::Purple), TextType::PurpleboiText);
	let grayboi_button = generate_cost_button(&mut commands, &ui_assets, -48.0, ButtonType::MoleculeButton(MoleculeType::Gray), TextType::GrayboiText);

	commands
		.entity(redboi_button)
		.insert(Name::new("Redboi Button Node"));

	commands
		.entity(blueboi_button)
		.insert(Name::new("Blueboi Button Node"));

	commands
		.entity(orangeboi_button)
		.insert(Name::new("Orangeboi Button Node"));

	commands
		.entity(purpleboi_button)
		.insert(Name::new("Purpleboi Button Node"));

	commands
		.entity(grayboi_button)
		.insert(Name::new("Grayboi Button Node"));

}

fn generate_cost_button (
	commands: &mut Commands,
	ui_assets: &Res<UiAssets>,
	height: f32,
	button_type: ButtonType,
	text_type: TextType,
) -> Entity {
	let button_style = Style {
		size: Size::new(Val::Px(384.0), Val::Px(84.0)),
		justify_content: JustifyContent::Center,
		align_items: AlignItems::Center,
		..default()
	};

	let button_text_style = TextStyle {
		font: ui_assets.font.clone(),
		font_size: 40.0,
		color: ui_assets.text_color,
	};

	commands
		.spawn_bundle(NodeBundle {
			style: Style {
				position_type: PositionType::Absolute,
				position: UiRect {
					// Screen width, sprite width, x translation
					left: Val::Px(1600.0 / 2.0 - 384.0 / 2.0 - 440.0),
					// Screen height, sprite height, y translation
					bottom: Val::Px(900.0 / 2.0 - 84.0 / 2.0 + height), 
					..default()
				},
				// Sprite width and height
				size: Size::new(Val::Px(300.0), Val::Px(84.0)),
				..default()
			},
			color: INVISIBLE.into(),
			..default()
		})
		.insert(ReactorUi)
		.with_children(|parent| {

			// Button
			parent
				.spawn_bundle(ButtonBundle {
					style: button_style.clone(),
					color: ui_assets.button_color.into(),
					..default()
				})
				.insert(button_type)
				
				.with_children(|parent| {

					// Button Sprite
					parent
						.spawn_bundle(ImageBundle {
							style: Style {
								size: Size::new(Val::Percent(100.0) , Val::Percent(100.0)),
								justify_content: JustifyContent::FlexStart,
								align_items: AlignItems::Center,
								..Default::default()
							},
							image: ui_assets.button_idle.clone().into(),
							..Default::default()
						})
						.insert(FocusPolicy::Pass)
						.insert(CostButton)
						.with_children(|parent| {

							// Text
							parent
								.spawn_bundle(TextBundle::from_section(
									"                        9000k",
									button_text_style.clone(),
								))
								.insert(text_type)
								;
						});
				});
		}).id()
}

fn generate_cost_button_sprites (
	mut commands: Commands,
	spritesheet: Res<SpriteSheets>,
    unlocked_query: Query<&Unlocked>,
) {
    let unlocked = unlocked_query.single();
    
	generate_cost_button_sprite(&mut commands, &spritesheet, SpriteType::Molecule(MoleculeType::Red), Vec3::new(-671.5, 320.0, 999.9), ButtonType::MoleculeButton(MoleculeType::Red), unlocked.red_unlocked);
	generate_cost_button_sprite(&mut commands, &spritesheet, SpriteType::Molecule(MoleculeType::Blue), Vec3::new(-671.5, 228.0, 999.9), ButtonType::MoleculeButton(MoleculeType::Blue), unlocked.blue_unlocked);
	generate_cost_button_sprite(&mut commands, &spritesheet, SpriteType::Molecule(MoleculeType::Orange), Vec3::new(-671.5, 136.0, 999.9), ButtonType::MoleculeButton(MoleculeType::Orange), unlocked.orange_unlocked);
	generate_cost_button_sprite(&mut commands, &spritesheet, SpriteType::Molecule(MoleculeType::Purple), Vec3::new(-671.5, 44.0, 999.9), ButtonType::MoleculeButton(MoleculeType::Purple), unlocked.purple_unlocked);
	generate_cost_button_sprite(&mut commands, &spritesheet, SpriteType::Molecule(MoleculeType::Gray), Vec3::new(-671.5, -48.0, 999.9), ButtonType::MoleculeButton(MoleculeType::Gray), unlocked.gray_unlocked);
}

fn generate_cost_button_sprite (
	mut commands: &mut Commands,
	spritesheet: &Res<SpriteSheets>,
	variant: SpriteType,
	location: Vec3,
	button_type: ButtonType,
    unlocked: bool,
) -> Entity {
	let sprite = spawn_spritesheet_sprite(
		&mut commands,
		&spritesheet,
		variant,
		if unlocked {0} else {8},
		Color::WHITE,
		location,
		Vec2::splat(64.0)
	);
	
	commands
	.entity(sprite)
	.insert(ReactorUi)
	.insert(CostButtonSprites)
	.insert(button_type)
	.id()
}

fn update_economy_text (
	economy_query: Query<(&Economy, &Clock, &Power)>,
	mut time_text_query: Query<&mut Text, (With<TimeText>, Without<PowerText>)>,
	mut power_text_query: Query<&mut Text, (With<PowerText>, Without<TimeText>)>,
    godmode_query: Query<&GodMode>,
) {
	let (_, time, power) = economy_query.single();
    let godmode = godmode_query.single();

	let mut time_text = time_text_query.single_mut();
	let current_time = ((1.0 - time.countdown.percent_left()) * 8.0 + 9.0).round();
    if godmode.0 {
        time_text.sections[0].value = format!("Chill");
    } else {
	    time_text.sections[0].value = format!("{current_time:.0}:00");
    }

	let mut power_text = power_text_query.single_mut();
	let mut current_power = power.current_power;

    if godmode.0 {
        power_text.sections[0].value = format!("999k");
    }
	else if current_power < 10.0 {
		power_text.sections[0].value = format!("{current_power:.2}");
	}
	else if current_power < 1000.0 {
		power_text.sections[0].value = format!("{current_power:.0}");
	}
    else if current_power < 10_000.0 {
        current_power /= 1000.0;
        power_text.sections[0].value = format!("{current_power:.2}k");
    }
	else if current_power < 1000_000.0 {
		current_power /= 1000.0;
		power_text.sections[0].value = format!("{current_power:.0}k");
	}
}


fn calculate_cost_text (
	cost_query: Query<&Cost>,
	mut boi_query: Query<(&mut Text, &TextType)>,
    unlocked_query: Query<&Unlocked>,
    godmode_query: Query<&GodMode>,
) {
    let unlocked = unlocked_query.single();
    let godmode = godmode_query.single();

	let cost = cost_query.single();
	
	let red_cost = cost.red_cost;
	let blue_cost = cost.blue_cost;
    let orange_cost = cost.orange_cost;
    let purple_cost = cost.purple_cost;
    let gray_cost = cost.gray_cost;

	for (text, text_type) in boi_query.iter_mut() {
		match text_type {
            // Red and blue always unlocked so unlocked check not necessary
			TextType::RedboiText => if godmode.0 {
                update_button_cost_text(0.0, text)
            } else if unlocked.get_unlocked(MoleculeType::Red) == &true {
                update_button_cost_text(red_cost, text)
            } else {
                update_button_cost_text(999000.0, text)
            },
			TextType::BlueboiText => if godmode.0 {
                update_button_cost_text(0.0, text)
            } else if unlocked.get_unlocked(MoleculeType::Blue) == &true {
                update_button_cost_text(blue_cost, text)
            } else {
                update_button_cost_text(999000.0, text)
            },
            TextType::OrangeboiText => if godmode.0 {
                update_button_cost_text(0.0, text)
            } else if unlocked.get_unlocked(MoleculeType::Orange) == &true {
                update_button_cost_text(orange_cost, text)
            } else {
                update_button_cost_text(999000.0, text)
            },
            TextType::PurpleboiText => if godmode.0 {
                update_button_cost_text(0.0, text)
            } else if unlocked.get_unlocked(MoleculeType::Purple) == &true {
                update_button_cost_text(purple_cost, text)
            } else {
                update_button_cost_text(999000.0, text)
            },
            TextType::GrayboiText => if godmode.0 {
                update_button_cost_text(0.0, text)
            } else if unlocked.get_unlocked(MoleculeType::Gray) == &true {
                update_button_cost_text(gray_cost, text)
            } else {
                update_button_cost_text(999000.0, text)
            },
		}
	}
}

fn update_button_cost_text (
	cost: f32,
	mut text: Mut<Text>,
) {

	if cost < 10.0 {
		text.sections[0].value = format!("                    {cost:.2}");
	}
	else if cost < 1000.0 {
		text.sections[0].value = format!("                    {cost:.0}");
	}
	else if cost < 100_000.0 {
		let kilo_cost = cost / 1000.0;
		text.sections[0].value = format!("                    {kilo_cost:.2}k");
	}
    else if cost < 1000_000.0 {
		let kilo_cost = cost / 1000.0;
		text.sections[0].value = format!("                    {kilo_cost:.0}k");
	}
}

fn update_button_cost_sprites (
	time: Res<Time>,
	mut animation_timer_query: Query<(&AnimationTimer, &mut BasicCountdown)>,
	mut button_monitor_query: Query<&mut ButtonHovered>,
	mut button_type_query: Query<(&ButtonType, &mut TextureAtlasSprite, With<CostButtonSprites>)>,
    unlocked_query: Query<&Unlocked>,
) {
	let (_animation_timer, mut countdown) = animation_timer_query.single_mut();
	let button_monitor = button_monitor_query.single_mut();
    let unlocked = unlocked_query.single();

    if countdown.0.tick(time.delta()).just_finished() {
	    for (button_type, mut sprite, _) in button_type_query.iter_mut() {
            if button_monitor.hovered_button == Some(*button_type) {
                match button_type {
                    &ButtonType::MoleculeButton(MoleculeType::Red) => {
                        if unlocked.red_unlocked == true {sprite.index = (sprite.index + 1) % 8;}
                        else {sprite.index = (sprite.index + 1) % 8 + 8;}
                    },
                    &ButtonType::MoleculeButton(MoleculeType::Blue) => {
                        if unlocked.blue_unlocked == true {sprite.index = (sprite.index + 1) % 8;}
                        else{sprite.index = (sprite.index + 1) % 8 + 8;}
                    },
                    &ButtonType::MoleculeButton(MoleculeType::Orange) => {
                        if unlocked.orange_unlocked == true {sprite.index = (sprite.index + 1) % 8;}
                        else{sprite.index = (sprite.index + 1) % 8 + 8;}
                    },
                    &ButtonType::MoleculeButton(MoleculeType::Purple) => {
                        if unlocked.purple_unlocked == true {sprite.index = (sprite.index + 1) % 8;}
                        else{sprite.index = (sprite.index + 1) % 8 + 8;}
                    },
                    &ButtonType::MoleculeButton(MoleculeType::Gray) => {
                        if unlocked.gray_unlocked == true {sprite.index = (sprite.index + 1) % 8;}
                        else{sprite.index = (sprite.index + 1) % 8 + 8;}
                    },
                }
            }
            else {
                match button_type {
                    &ButtonType::MoleculeButton(MoleculeType::Red) => {
                        if unlocked.red_unlocked == true {sprite.index = 0;}
                        else {sprite.index = 8;}
                    },
                    &ButtonType::MoleculeButton(MoleculeType::Blue) => {
                        if unlocked.blue_unlocked == true {sprite.index = 0;}
                        else{sprite.index = 8;}
                    },
                    &ButtonType::MoleculeButton(MoleculeType::Orange) => {
                        if unlocked.orange_unlocked == true {sprite.index = 0;}
                        else{sprite.index = 8;}
                    },
                    &ButtonType::MoleculeButton(MoleculeType::Purple) => {
                        if unlocked.purple_unlocked == true {sprite.index = 0;}
                        else{sprite.index = 8;}
                    },
                    &ButtonType::MoleculeButton(MoleculeType::Gray) => {
                        if unlocked.gray_unlocked == true {sprite.index = 0;}
                        else{sprite.index = 8;}
                    },
                }
            }
        }
	}
}

// Maybe redundant, just edit transform directly?
fn update_reactor_size (
	mut reactor_query: Query<(&Reactor, &Dimensions, &mut Transform)>,
) {
	// Remove single mut when adding more reactors
	let (_reactor, dimensions, mut transform) = reactor_query.single_mut();
	// 100.0 is sprite width and height (check generate_reactor)
	transform.scale.x = dimensions.x_size / 100.0;
	transform.scale.y = dimensions.y_size / 100.0;
}

// This function is TOO BIG -hakan
// Lmao yeah, why is it even named button cost? -iq
// All the button functions are called "button_function"
// This one is the button that displays the cost of molecules
// And calls their spawn function
fn button_cost (
	mut commands: Commands,
	spritesheet: Res<SpriteSheets>,
	mut cost_query: Query<&mut Cost>,
	mut interaction_query: Query<(&ButtonType, &Children, &Interaction), Changed<Interaction>>,
	mut molecule_list_query: Query<&MoleculeList>,
	reactor_query: Query<(&Reactor, &Dimensions, &Transform)>,
	mut economy_query: Query<(&Economy, &mut Power)>,
	mut button_image_query: Query<&mut UiImage>,
	ui_assets: Res<UiAssets>,
	mut button_monitor_query: Query<&mut ButtonHovered>,
    unlocked_query: Query<&Unlocked>,
    godmode_query: Query<&GodMode>,
) {
	let (_economy, mut power) = economy_query.single_mut();
	let mut cost = cost_query.single_mut();
	let mut button_monitor = button_monitor_query.single_mut();
    let unlocked = unlocked_query.single();
    let godmode = godmode_query.single();

	for (button_type, children, interaction) in interaction_query.iter_mut() {
		let child = children.iter().next().unwrap();
		let mut image = button_image_query.get_mut(*child).unwrap();

		match interaction {
			Interaction::Clicked => {
				image.0 = ui_assets.button_hovered.clone();
				match button_type {
					ButtonType::MoleculeButton(molecule_type) => {
						let molecule_cost = cost.get_cost_mut(*molecule_type);
                        if unlocked.get_unlocked(*molecule_type) == &true {
                            if power.current_power > *molecule_cost || godmode.0{
                                add_molecule(*molecule_type, &mut commands, &spritesheet, &mut molecule_list_query, &reactor_query);
                                power.current_power -= *molecule_cost;
                                *molecule_cost *= COST_SCALING;
                            };
                        }
					}
				}
			}
			Interaction::Hovered => {
				image.0 = ui_assets.button_hovered.clone();
				button_monitor.hovered_button = Some(*button_type);
			} 
			Interaction::None => {
				image.0 = ui_assets.button_idle.clone();
				if Some(*button_type) == button_monitor.hovered_button {
					button_monitor.hovered_button = None;
				}
			}
		}
	}
}

// Change to V for vent???
// sus
fn button_control_rod (
	commands: Commands,
	keyboard: Res<Input<KeyCode>>,
	entities: Query<Entity, With<Molecule>>,
) {
	if keyboard.just_pressed(KeyCode::Escape) {
		add_control_rod(commands, entities);
	}
}

fn button_standby (
	mut interaction_query: Query<(&StandbyButton, &Interaction), Changed<Interaction>>,
	mut state: ResMut<State<GameState>>,
) {
	for (_, interaction) in interaction_query.iter_mut() {
		match interaction {
			Interaction::Clicked => {
				state.pop().expect("Failed to change states");
			}
			Interaction::Hovered => {
			} 
			Interaction::None => {
			}
		}
	}
}
