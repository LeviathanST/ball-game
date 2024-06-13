use bevy::{app::AppExit, input::ButtonInput, prelude::{EventWriter, KeyCode, Res}};

pub fn exit_game (
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_writer_event: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_writer_event.send(AppExit);
    }
}