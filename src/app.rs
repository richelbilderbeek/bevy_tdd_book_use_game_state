//! This example illustrates how to use [`States`] for high-level app control flow.
//! States are a powerful but intuitive tool for controlling which logic runs when.
//! You can have multiple independent states, and the [`OnEnter`] and [`OnExit`] schedules
//! can be used to great effect to ensure that you handle setup and teardown appropriately.
//!
//! In this case, we're transitioning from a `Menu` state to an `InGame` state.


use bevy::prelude::*;

//use bevy_dev_tools::states::*;

// Copied from bevy_dev_tools::states
pub fn log_transitions<S: States>(mut transitions: EventReader<StateTransitionEvent<S>>) {
    // State internals can generate at most one event (of type) per frame.
    let Some(transition) = transitions.read().last() else {
        return;
    };
    let name = std::any::type_name::<S>();
    let StateTransitionEvent { exited, entered } = transition;
    info!("{} transition: {:?} => {:?}", name, exited, entered);
}

pub fn create_app() -> App {
    let mut app = App::new();

    // Only add this plugin in testing.
    // The main app will assume it to be absent
    if cfg!(test) {
        app.add_plugins(MinimalPlugins);
        //app.add_plugins(InputPlugin);
        //app.add_plugins(ScheduleRunnerPlugin::default());
        app.add_plugins(bevy::state::app::StatesPlugin);
    } else {
        app.add_plugins(DefaultPlugins);
    }

    app
        //.add_plugins(DefaultPlugins)
        .init_state::<AppState>() // Alternatively we could use .insert_state(AppState::Menu)
        .add_systems(Startup, setup)

        .add_systems(OnEnter(AppState::Menu), add_menu_text)
        .add_systems(OnEnter(AppState::InGame), add_game_text)

        .add_systems(Update, menu_respond_to_keyboard.run_if(in_state(AppState::Menu)))
        .add_systems(Update, in_game_respond_to_keyboard.run_if(in_state(AppState::InGame)))

        .add_systems(OnExit(AppState::Menu), despawn_all_text)
        .add_systems(OnExit(AppState::InGame), despawn_all_text)

        .add_systems(Update, log_transitions::<AppState>);

    app
}

fn menu_respond_to_keyboard(
    input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,

) {
    if input.just_pressed(KeyCode::Space) {
        next_state.set(AppState::InGame);
    }
}

fn in_game_respond_to_keyboard(
    input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,

) {
    if input.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::Menu);
    }
}

fn add_game_text(mut commands: Commands) {
    commands.spawn(Text2dBundle {
        text: Text::from_section(String::from("Game. Press escape to quit"), TextStyle { ..default() }),
        ..default()
    });
}

fn add_menu_text(mut commands: Commands) {
    commands.spawn(Text2dBundle {
        text: Text::from_section(String::from("Menu. Press space to start"), TextStyle { ..default() }),
        ..default()
    });
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Menu,
    InGame,
}

#[cfg(test)]
fn count_n_texts(app: &mut App) -> usize {
    let mut query = app.world_mut().query::<&Text>();
    return query.iter(app.world()).len();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn despawn_all_text(
    mut commands: Commands,
    query: Query<Entity, With<Text>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_app_has_text() {
        let mut app = App::new();
        app.update();
        assert_eq!(count_n_texts(&mut app), 0);
    }

    #[test]
    fn test_app_has_text() {
        let mut app = create_app();
        app.update();
        assert_eq!(count_n_texts(&mut app), 1);
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