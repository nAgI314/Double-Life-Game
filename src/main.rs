use bevy::{prelude::{App, Camera2d, Commands, DefaultPlugins,Startup, }};
use bevy_state::{app::AppExtStates, state::States};

use crate::{in_game::InGamePlugin, main_menu::MainMenuPlugin};

pub mod main_menu;
pub mod in_game;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    MainMenu,
    InGame,
}

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_plugins(MainMenuPlugin)
        .add_plugins(InGamePlugin)
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}