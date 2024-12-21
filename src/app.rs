//! This example illustrates how to use [`States`] for high-level app control flow.
//! States are a powerful but intuitive tool for controlling which logic runs when.
//! You can have multiple independent states, and the [`OnEnter`] and [`OnExit`] schedules
//! can be used to great effect to ensure that you handle setup and teardown appropriately.
//!
//! In this case, we're transitioning from a `Menu` state to an `InGame` state.


use bevy::prelude::*;

use bevy::input::InputPlugin;

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

    // The function 'try_add_plugins' 
    // (https://github.com/bevyengine/bevy/discussions/15802#discussioncomment-10898148)
    // will make this if obsolete and increase code coverage.
    // Thanks mgi388 for pointing this out
    if cfg!(test) {
        app.add_plugins(MinimalPlugins);
        app.add_plugins(InputPlugin);
        app.add_plugins(bevy::state::app::StatesPlugin);
    } else {
        app.add_plugins(DefaultPlugins);
    }

    app
        .init_state::<AppState>()
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
    commands.spawn(Text2d {
        text: Text::from_section(String::from("Game. Press escape to quit"), TextStyle { ..default() }),
        ..default()
    });
}

fn add_menu_text(mut commands: Commands) {
    commands.spawn(Text2d {
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
    let mut query = app.world_mut().query::<&Text2d>();
    return query.iter(app.world()).len();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn despawn_all_text(
    mut commands: Commands,
    query: Query<Entity, With<Text2d>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}


#[cfg(test)]
fn get_text(app: &mut App) -> String {
    assert_eq!(count_n_texts(app), 1);
    let mut query = app.world_mut().query::<&Text2d>();
    return query.single(app.world_mut()).sections[0].value.clone();
}

#[cfg(test)]
fn get_program_state(app: &mut App) -> AppState {
    return *app.world_mut().resource_mut::<State<AppState>>().get()
    /*
    *world. resource_mut::<State<GameState>>()
    app.
    let mut query = app.world_mut().query::<&Text2d>();
    return query.single(app.world_mut()).sections[0].value.clone();

    game_state: Res<State<GameState>>) {
        match game_state. get() {
            GameState::InGame => {
                // Run game logic here...
            },
            _ => {},
  */
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

    #[test]
    fn test_app_has_menu_text() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_text(&mut app), "Menu. Press space to start");
    }

    #[test]
    fn test_app_starts_at_menu() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_program_state(&mut app), AppState::Menu);
    }

    #[test]
    fn test_space_starts_game() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_program_state(&mut app), AppState::Menu);
        app.world_mut()
            .send_event(bevy::input::keyboard::KeyboardInput {
                key_code: KeyCode::Space,
                logical_key: bevy::input::keyboard::Key::Space,
                state: bevy::input::ButtonState::Pressed,
                window: Entity::PLACEHOLDER,
            });
        app.update();
        app.update();
        assert_eq!(get_program_state(&mut app), AppState::InGame);
    }

    #[test]
    fn test_escape_leaves_game() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_program_state(&mut app), AppState::Menu);
        app.world_mut()
            .send_event(bevy::input::keyboard::KeyboardInput {
                key_code: KeyCode::Space,
                logical_key: bevy::input::keyboard::Key::Space,
                state: bevy::input::ButtonState::Pressed,
                window: Entity::PLACEHOLDER,
            });
        app.update();
        app.update();
        assert_eq!(get_program_state(&mut app), AppState::InGame);
        app.world_mut()
            .send_event(bevy::input::keyboard::KeyboardInput {
                key_code: KeyCode::Escape,
                logical_key: bevy::input::keyboard::Key::Escape,
                state: bevy::input::ButtonState::Pressed,
                window: Entity::PLACEHOLDER,
            });
        app.update();
        app.update();
        assert_eq!(get_program_state(&mut app), AppState::Menu);
    }

}
