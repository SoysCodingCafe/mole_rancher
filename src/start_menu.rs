use bevy::{prelude::*, ui::FocusPolicy, app::AppExit};

use crate::{{GameState, INVISIBLE}, audio::Volume, reactor::generate_unlock_list};

// ---------- PLUGINS ----------
pub struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
	fn build(&self, app: &mut App) {
		app
		.add_startup_system_to_stage(StartupStage::PreStartup, load_menu_assets)
        .add_startup_system(generate_unlock_list)
		// START MENU
		.add_system_set(SystemSet::on_enter(GameState::StartMenu)
			.with_system(load_menu)
		)
		.add_system_set(SystemSet::on_update(GameState::StartMenu)
			.with_system(show_buttons)
			.with_system(button_start)
			.with_system(button_settings)
			.with_system(button_quit)
		)
		.add_system_set(SystemSet::on_pause(GameState::StartMenu)
			.with_system(despawn_screen::<StartMenuUi>)
		)
		.add_system_set(SystemSet::on_resume(GameState::StartMenu)
			.with_system(load_menu)
		)
		// SETTINGS MENU
		.add_system_set(SystemSet::on_enter(GameState::SettingsMenu)
			.with_system(load_settings)
		)
        .add_system_set(SystemSet::on_resume(GameState::SettingsMenu)
			.with_system(load_settings)
		)
		.add_system_set(SystemSet::on_update(GameState::SettingsMenu)
			.with_system(show_buttons)
            .with_system(button_audio_settings)
			.with_system(button_return_to_start_menu)
		)
		.add_system_set(SystemSet::on_exit(GameState::SettingsMenu)
			.with_system(despawn_screen::<SettingsMenuUi>)
		)
        .add_system_set(SystemSet::on_pause(GameState::SettingsMenu)
			.with_system(despawn_screen::<SettingsMenuUi>)
		)
        // AUDIO SETTINGS MENU
        .add_system_set(SystemSet::on_enter(GameState::SettingsAudioMenu)
			.with_system(load_settings_audio)
		)
        .add_system_set(SystemSet::on_update(GameState::SettingsAudioMenu)
            .with_system(show_buttons)
            .with_system(button_system)
			.with_system(setting_button::<Volume>)
			.with_system(button_return_to_settings)
		)
        .add_system_set(SystemSet::on_exit(GameState::SettingsAudioMenu)
			.with_system(despawn_screen::<SettingsAudioMenuUi>)
		)
		;
	}
}

// ---------- RESOURCES ----------
pub struct UiAssets {
	pub font: Handle<Font>,
	pub text_color: Color,
	pub button_color: Color,
	pub button_idle: Handle<Image>,
	pub button_hovered: Handle<Image>,
}

// ---------- COMPONENTS ---------
// Tag all entities on menus for easy cleanup
#[derive(Component)]
struct StartMenuUi;

#[derive(Component)]
struct SettingsMenuUi;

#[derive(Component)]
struct SettingsAudioMenuUi;

// Tags for button logic
#[derive(Component)]
struct UiButton;

#[derive(Component)]
struct StartButton;

#[derive(Component)]
struct SettingsButton;

#[derive(Component)]
struct SettingsAudioButton;

#[derive(Component)]
struct VolumeControlButton;

#[derive(Component)]
struct ReturnToSettingsButton;

#[derive(Component)]
struct ReturnToStartMenuButton;

#[derive(Component)]
struct QuitButton;

// Tag component for volume setting
#[derive(Component)]
struct SelectedOption;

// List of actions that can be trigger by buttons
// Still need to implement match statement for enum
// To simplify interaction systems for each button
#[derive(Component)]
enum MenuButtonAction {
	Start,
	Settings,
	SettingsAudio,
	ReturnToSettings,
	ReturnToStartMenu,
	Quit,
}

// ---------- SYSTEMS ----------

// MENU SYSTEMS
fn load_menu_assets(
	mut commands: Commands,
	asset_server: Res<AssetServer>
)
{
	let ui_assets = UiAssets {
		font: asset_server.load("fonts/PixelSplitterBold.ttf"),
		text_color: Color::rgb(0.0, 0.3, 0.0),
		button_color: INVISIBLE,
		button_idle: asset_server.load("buttons/button_idle.png"),
		button_hovered: asset_server.load("buttons/button_hovered.png")
	};

	commands.insert_resource(ui_assets);
}

fn load_menu(
	mut commands: Commands,
	ui_assets: Res<UiAssets>,
) {
	let button_style = Style {
		size: Size::new(Val::Px(250.0), Val::Px(65.0)),
		margin: UiRect::all(Val::Px(20.0)),
		justify_content: JustifyContent::Center,
		align_items: AlignItems::Center,
		..default()
	};

	let button_text_style = TextStyle {
		font: ui_assets.font.clone(),
		font_size: 32.0,
		color: ui_assets.text_color,
	};

	commands
			// Node Bundle to hold the Start Menu UI elements
			.spawn_bundle(NodeBundle {
				style: Style {
					margin: UiRect::all(Val::Auto),
					flex_direction: FlexDirection::ColumnReverse,
					align_items: AlignItems::Center,
					..default()
				},
				color: INVISIBLE.into(),
				..default()
			})
			.insert(StartMenuUi)
			.insert(Name::new("Start Menu UI"))
			.with_children(|parent| {

				// Game Title
				parent.spawn_bundle(
					TextBundle::from_section(
						"Mole Rancher",
						TextStyle {
							font: ui_assets.font.clone(),
							font_size: 200.0,
							color: Color::rgb(0.0, 0.3, 0.0),
						},
					)
					.with_style(Style {
						margin: UiRect::all(Val::Px(50.0)),
						..default()
					}),
				);

				// Lab Button
				parent
					.spawn_bundle(ButtonBundle {
						style: button_style.clone(),
						color: ui_assets.button_color.into(),
						..default()
					})
					.insert(MenuButtonAction::Start)
					.insert(StartButton)
					.insert(UiButton)
					.with_children(|parent| {

						// Lab Button Sprite
						parent
							.spawn_bundle( ImageBundle {
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

							// Lab Button Text
							parent.spawn_bundle(TextBundle::from_section(
								"Enter the Lab",
								button_text_style.clone(),
							));
						});
					});

				// Settings Button
				parent
					.spawn_bundle(ButtonBundle {
						style: button_style.clone(),
						color: ui_assets.button_color.into(),
						..default()
					})
					.insert(MenuButtonAction::Settings)
					.insert(SettingsButton)
					.insert(UiButton)
					.with_children(|parent| {

						// Settings Button Sprite
						parent
							.spawn_bundle( ImageBundle {
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

							// Settings Button Text
							parent.spawn_bundle(TextBundle::from_section(
								"Settings",
								button_text_style.clone(),
							));
						});
					});

				// Quit Button
				parent
					.spawn_bundle(ButtonBundle {
						style: button_style,
						color: ui_assets.button_color.into(),
						..default()
					})
					.insert(MenuButtonAction::Quit)
					.insert(QuitButton)
					.insert(UiButton)
					.with_children(|parent| {

						// Quit Button Sprite
						parent
							.spawn_bundle( ImageBundle {
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

							// Quit Button Text
							parent.spawn_bundle(TextBundle::from_section(
								"Quit",
								button_text_style.clone(),
							));
						});
					});
			});
}

fn load_settings (
	mut commands: Commands,
	ui_assets: Res<UiAssets>,
) {
	let button_style = Style {
		size: Size::new(Val::Px(250.0), Val::Px(65.0)),
		margin: UiRect::all(Val::Px(20.0)),
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
		// Node Bundle to hold the Start Menu UI elements
		.spawn_bundle(NodeBundle {
			style: Style {
				margin: UiRect::all(Val::Auto),
				flex_direction: FlexDirection::ColumnReverse,
				align_items: AlignItems::Center,
				..default()
			},
			color: INVISIBLE.into(),
			..default()
		})
		.insert(SettingsMenuUi)
		.insert(Name::new("Settings Menu UI"))
		.with_children(|parent| {

				// Audio Settings Button
				parent
					.spawn_bundle(ButtonBundle {
						style: button_style.clone(),
						color: ui_assets.button_color.into(),
						..default()
					})
					.insert(MenuButtonAction::SettingsAudio)
					.insert(SettingsAudioButton)
					.insert(UiButton)
					.with_children(|parent| {

						// Audio Settings Button Sprite
						parent
							.spawn_bundle(ImageBundle {
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
							parent
								.spawn_bundle(TextBundle::from_section(
									"Audio",
									button_text_style.clone(),
								));
						});
					});
		})


		.with_children(|parent| {

			// Return to Start Menu Button
			parent
				.spawn_bundle(ButtonBundle {
					style: button_style.clone(),
					color: ui_assets.button_color.into(),
					..default()
				})
				.insert(MenuButtonAction::ReturnToStartMenu)
				.insert(ReturnToStartMenuButton)
				.insert(UiButton)
				.with_children(|parent| {

					// Return to Start Menu Button Sprite
					parent
						.spawn_bundle(ImageBundle {
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
						parent
							.spawn_bundle(TextBundle::from_section(
								"Back",
								button_text_style.clone(),
							));
					});
				});
		});
}

fn load_settings_audio (
    mut commands: Commands,
    volume: Res<Volume>,
    ui_assets: Res<UiAssets>,
) {
    let button_style = Style {
		size: Size::new(Val::Px(250.0), Val::Px(65.0)),
		margin: UiRect::all(Val::Px(20.0)),
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
                    margin: UiRect::all(Val::Auto),
                    flex_direction: FlexDirection::ColumnReverse,
                    align_items: AlignItems::Center,
                    ..default()
                },
                color: INVISIBLE.into(),
                ..default()
            })
            .insert(SettingsAudioMenuUi)
            .with_children(|parent| {
                parent
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        color: Color::rgb(0.4, 0.9, 0.4).into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle::from_section(
                            "Volume",
                            button_text_style.clone(),
                        ));
                        for volume_setting in [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9] {
                            let mut entity = parent.spawn_bundle(ButtonBundle {
                                style: Style {
                                    size: Size::new(Val::Px(30.0), Val::Px(65.0)),
                                    ..button_style.clone()
                                },
                                color: Color::GREEN.into(),
                                ..default()
                            });
                            entity.insert(Volume{
								bgm: volume_setting,
                                sfx: volume_setting,
							});
                            if *volume == (Volume{
                                bgm: volume_setting,
                                sfx: volume_setting}) {
                                entity
                                .insert(SelectedOption);
                            }
                        }
                    });
                    parent
                        .spawn_bundle(ButtonBundle {
                            style: button_style.clone(),
                            color: ui_assets.button_color.into(),
                            ..default()
                        })
                        .insert(MenuButtonAction::ReturnToSettings)
                        .insert(ReturnToSettingsButton)
                        .insert(UiButton)
                        .with_children(|parent| {
        
                            // Return to Settings Button Sprite
                            parent
                                .spawn_bundle(ImageBundle {
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
                                parent
                                    .spawn_bundle(TextBundle::from_section(
                                        "Back",
                                        button_text_style.clone(),
                                    ));
                            });
                        });
            });
}

// BUTTON SYSTEMS
fn show_buttons (
	mut interaction_query: Query<(&UiButton, &Children, &Interaction), Changed<Interaction>>,
	mut image_query: Query<&mut UiImage>,
	ui_assets: Res<UiAssets>,
	
) {
	for (_, children, interaction) in interaction_query.iter_mut() {
		let child = children.iter().next().unwrap();
		let mut image = image_query.get_mut(*child).unwrap();

		match interaction {
			Interaction::Hovered | Interaction::Clicked => {
				image.0 = ui_assets.button_hovered.clone();
			}
			Interaction::None => {
				image.0 = ui_assets.button_idle.clone();
			}
		}
	}
}

fn button_start (
	mut interaction_query: Query<(&StartButton, &Interaction), Changed<Interaction>>,
	mut state: ResMut<State<GameState>>,
) {
	for (_, interaction) in interaction_query.iter_mut() {
		match interaction {
			Interaction::Clicked => {
				state.push(GameState::Lab).expect("Failed to change states");
			}
			Interaction::Hovered | Interaction::None => {
			}
		}
	}
}

fn button_settings (
	mut interaction_query: Query<(&SettingsButton, &Interaction), Changed<Interaction>>,
	mut state: ResMut<State<GameState>>,
) {
	for (_, interaction) in interaction_query.iter_mut() {
		match interaction {
			Interaction::Clicked => {
				state.push(GameState::SettingsMenu).expect("Failed to change states");
			}
			Interaction::Hovered | Interaction::None => {
			}
		}
	}
}

fn button_audio_settings (
    mut interaction_query: Query<(&SettingsAudioButton, &Interaction), Changed<Interaction>>,
	mut state: ResMut<State<GameState>>,
) {
    for (_, interaction) in interaction_query.iter_mut() {
		match interaction {
			Interaction::Clicked => {
				state.push(GameState::SettingsAudioMenu).expect("Failed to change states");
			}
			Interaction::Hovered | Interaction::None => {
			}
		}
	}
}

// AUDIO SETTING SYSTEMS
fn button_system(
	mut interaction_query: Query<
		(&Interaction, &mut UiColor, Option<&SelectedOption>),
		(Changed<Interaction>, With<Button>, Without<ReturnToSettingsButton>),
	>,
) {
	for (interaction, mut color, selected) in &mut interaction_query {
		*color = match (*interaction, selected) {
			(Interaction::Clicked, _) | (Interaction::None, Some(_)) => Color::RED.into(),
			(Interaction::Hovered, Some(_)) => Color::RED.into(),
			(Interaction::Hovered, None) => Color::RED.into(),
			(Interaction::None, None) => Color::GREEN.into(),
		}
	}
}

fn setting_button<T: Component + PartialEq + Copy>(
	interaction_query: Query<(&Interaction, &T, Entity), (Changed<Interaction>, With<Button>)>,
	mut selected_query: Query<(Entity, &mut UiColor), With<SelectedOption>>,
	mut commands: Commands,
	mut setting: ResMut<T>,
) {
	for (interaction, button_setting, entity) in &interaction_query {
		if *interaction == Interaction::Clicked && *setting != *button_setting {
			let (previous_button, mut previous_color) = selected_query.single_mut();
			*previous_color = Color::GREEN.into();
			commands.entity(previous_button).remove::<SelectedOption>();
			commands.entity(entity).insert(SelectedOption);
			*setting = *button_setting;
		}
	}
}

fn button_return_to_settings (
    mut interaction_query: Query<(&ReturnToSettingsButton, &Interaction), Changed<Interaction>>,
	mut state: ResMut<State<GameState>>,
) {
    for (_, interaction) in interaction_query.iter_mut() {
		match interaction {
			Interaction::Clicked => {
				state.pop().expect("Failed to change states");
			}
			Interaction::Hovered | Interaction::None => {
			}
		}
	}
}

fn button_quit (
	mut interaction_query: Query<(&QuitButton, &Interaction), Changed<Interaction>>,
	mut exit: EventWriter<AppExit>,
) {
	for (_, interaction) in interaction_query.iter_mut() {
		match interaction {
			Interaction::Clicked => {
				exit.send(AppExit);
			}
			Interaction::Hovered | Interaction::None => {
			}
		}
	}
}

fn button_return_to_start_menu (
	mut interaction_query: Query<(&ReturnToStartMenuButton, &Interaction), Changed<Interaction>>,
	mut state: ResMut<State<GameState>>,
) {
	for (_, interaction) in interaction_query.iter_mut() {
		match interaction {
			Interaction::Clicked => {
				state.pop().expect("Failed to change states");
			}
			Interaction::Hovered | Interaction::None => {
			}
		}
	}
}

// CLEANUP SYSTEMS
pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
	for entity in &to_despawn {
		commands.entity(entity).despawn_recursive();
	}
}
