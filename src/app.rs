//! This example illustrates how to use [`States`] for high-level app control flow.
//! States are a powerful but intuitive tool for controlling which logic runs when.
//! You can have multiple independent states, and the [`OnEnter`] and [`OnExit`] schedules
//! can be used to great effect to ensure that you handle setup and teardown appropriately.
//!
//! In this case, we're transitioning from a `Menu` state to an `InGame` state.

use bevy::{dev_tools::states::*, prelude::*};

pub fn create_app() -> App {
    let mut app = App::new();

    app
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>() // Alternatively we could use .insert_state(AppState::Menu)
        .add_systems(Startup, setup)
        // This system runs when we enter `AppState::Menu`, during the `StateTransition` schedule.
        // All systems from the exit schedule of the state we're leaving are run first,
        // and then all systems from the enter schedule of the state we're entering are run second.
        .add_systems(OnEnter(AppState::Menu), setup_menu)
        // By contrast, update systems are stored in the `Update` schedule. They simply
        // check the value of the `State<T>` resource to see if they should run each frame.
        .add_systems(Update, menu.run_if(in_state(AppState::Menu)))
        .add_systems(OnExit(AppState::Menu), cleanup_menu)
        .add_systems(OnEnter(AppState::InGame), setup_game)
        .add_systems(
            Update,
            (movement, change_color).run_if(in_state(AppState::InGame)),
        )
        .add_systems(Update, log_transitions::<AppState>);

    app
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Menu,
    InGame,
}

#[derive(Resource)]
struct MenuData {
    button_entity: Entity,
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_menu(mut commands: Commands) {
    let button_entity = commands
        .spawn(NodeBundle {
            style: Style {
                // center button
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.),
                        height: Val::Px(65.),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font_size: 33.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        })
        .id();
    commands.insert_resource(MenuData { button_entity });
}

fn menu(
    mut next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                next_state.set(AppState::InGame);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.button_entity).despawn_recursive();
}

fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/icon.png"),
        ..default()
    });
}

const SPEED: f32 = 100.0;
fn movement(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Sprite>>,
) {
    for mut transform in &mut query {
        let mut direction = Vec3::ZERO;
        if input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if input.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }
        if input.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if input.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }

        if direction != Vec3::ZERO {
            transform.translation += direction.normalize() * SPEED * time.delta_seconds();
        }
    }
}

fn change_color(time: Res<Time>, mut query: Query<&mut Sprite>) {
    for mut sprite in &mut query {
        let new_color = LinearRgba {
            blue: ops::sin(time.elapsed_seconds() * 0.5) + 2.0,
            ..LinearRgba::from(sprite.color)
        };

        sprite.color = new_color.into();
    }
}

/*

use bevy::prelude::*;

use bevy::prelude::States;

#[derive(States, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum ProgramState {
    #[default]
    MainMenu,
    InGame,
}

pub fn create_app() -> App {
    let mut app = App::new();

    // Only add these plugin in testing.
    // The main app will assume it to be absent.
    // Adding DefaultPlugins will cause tests to crash
    if cfg!(test) {
        app.add_plugins(bevy::state::app::StatesPlugin);
    } else {
        app.add_plugins(DefaultPlugins);
    }
    app.init_state::<ProgramState>(); // Crashes here

    app.add_systems(
        Startup,
        add_text_in_main_menu.run_if(in_state(ProgramState::MainMenu)),
    );
    app.add_systems(
        Startup,
        add_text_in_game.run_if(in_state(ProgramState::InGame)),
    );
    app.add_systems(
        Update,
        respond_to_keys_in_main_menu.run_if(in_state(ProgramState::MainMenu)),
    );
    app.add_systems(
        Update,
        respond_to_keys_in_game.run_if(in_state(ProgramState::InGame)),
    );
    // NO! Do not update!
    // text will be invisible in main
    //app.update();


    app
}

fn respond_to_keys_in_game(
    state: Res<State<ProgramState>>,
    mut next_state: ResMut<NextState<ProgramState>>,
    input: ResMut<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        assert_eq!(state.get().clone(), ProgramState::InGame);
        next_state.set(ProgramState::MainMenu);
    }
}

fn respond_to_keys_in_main_menu(
    state: Res<State<ProgramState>>,
    mut next_state: ResMut<NextState<ProgramState>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        assert_eq!(state.get().clone(), ProgramState::MainMenu);
        next_state.set(ProgramState::InGame);
    }
}

fn add_text_in_main_menu(mut commands: Commands) {
    commands.spawn(Text2dBundle {
        text: Text::from_section("Welcome to the main menu", TextStyle { ..default() }),
        ..default()
    });
}

fn add_text_in_game(mut commands: Commands) {
    commands.spawn(Text2dBundle {
        text: Text::from_section("In the game!", TextStyle { ..default() }),
        ..default()
    });
}
#[cfg(test)]
fn count_n_texts(app: &mut App) -> usize {
    let mut query = app.world_mut().query::<&Text>();
    return query.iter(app.world()).len();
}

#[cfg(test)]
fn get_program_state(app: &mut App) -> ProgramState {
    app.world().resource::<State<ProgramState>>().get().clone()
}

#[cfg(test)]
fn get_text(app: &mut App) -> String {
    let mut query = app.world_mut().query::<&Text>();
    return query.single(app.world_mut()).sections[0].value.clone();
}

#[cfg(test)]
mod tests {
    use bevy::input::keyboard::Key;
    use super::*;

    #[test]
    fn test_empty_app_has_text() {
        let mut app = App::new();
        app.update();
        assert_eq!(count_n_texts(&mut app), 0);
    }

    #[test]
    fn test_can_create_app() {
        create_app();
    }

    #[test]
    fn test_app_has_text() {
        let mut app = create_app();
        app.update();
        assert_eq!(count_n_texts(&mut app), 1);
    }

    #[test]
    fn test_app_uses_text() {
        let mut app = create_app();
        app.update();
        assert!(get_text(&mut app).len() > 0);
    }

    #[test]
    fn test_app_starts_at_menu() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_program_state(&mut app), ProgramState::MainMenu);
    }
    #[test]
    fn test_app_starts_game() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_program_state(&mut app), ProgramState::MainMenu);

        // Press the space button, thanks kristoff3r
        app.world_mut()
            .send_event(bevy::input::keyboard::KeyboardInput {
                key_code: KeyCode::Space,
                logical_key: bevy::input::keyboard::Key::Space,
                state: bevy::input::ButtonState::Pressed,
                window: Entity::PLACEHOLDER,
            });

        app.update();

        assert_eq!(get_program_state(&mut app), ProgramState::InGame);
        assert_eq!(1, 2);

    }
}
*/