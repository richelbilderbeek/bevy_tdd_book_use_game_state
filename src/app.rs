use bevy::prelude::*;

pub fn create_app(text: String) -> App {
    let mut app = App::new();

    let add_text_fn = move |commands: Commands| add_text(commands, &text);
    app.add_systems(Startup, add_text_fn);

    // NO! Do not update!
    // text will be invisible in main
    //app.update();
    app
}

fn add_text(mut commands: Commands, str: &String) {
    commands.spawn(Text2dBundle {
        text: Text::from_section(str, TextStyle { ..default() }),
        ..default()
    });
}

#[cfg(test)]
fn count_n_texts(app: &mut App) -> usize {
    let mut query = app.world_mut().query::<&Text>();
    return query.iter(app.world()).len();
}

#[cfg(test)]
fn get_text(app: &mut App) -> String {
    let mut query = app.world_mut().query::<&Text>();
    return query.single(app.world_mut()).sections[0].value.clone();
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
    fn test_can_create_app_from_str() {
        create_app(String::from("irrelevant"));
    }

    #[test]
    fn test_app_has_text() {
        let mut app = create_app(String::from("irrelevant"));
        app.update();
        assert_eq!(count_n_texts(&mut app), 1);
    }

    #[test]
    fn test_app_uses_text() {
        let text = String::from("some random text");
        let mut app = create_app(text.clone());
        app.update();
        assert_eq!(get_text(&mut app), text);
    }
}
