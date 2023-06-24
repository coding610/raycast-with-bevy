use bevy::prelude::*;
use bevy::window::{PresentMode, PrimaryWindow, WindowMode, CursorGrabMode};

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        }
    );
}

fn setup_window(mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = window_query.single_mut();
    window.title = format!("La raycaster");
    window.present_mode = PresentMode::AutoVsync;
    println!("res: {:#?}", window.resolution);
    // window.cursor.visible = false;
    // window.resizable = false;
    // window.mode = WindowMode::BorderlessFullscreen;
    // window.cursor.grab_mode = CursorGrabMode::Locked;
}

pub struct CorePlugin;
impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera)
            .add_startup_system(setup_window);
    }
}
