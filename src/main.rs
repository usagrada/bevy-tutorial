// use bevy::prelude::AddAsset
extern crate bevy;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

pub const FONT: &str = "fonts/Mplus2-Black.ttf";
pub const UNIT_WIDTH: u32 = 40;
pub const UNIT_HEIGHT: u32 = 40;

pub const X_LENGTH: u32 = 10;
pub const Y_LENGTH: u32 = 18;

pub const SCREEN_WIDTH: u32 = UNIT_WIDTH * X_LENGTH;
pub const SCREEN_HEIGHT: u32 = UNIT_HEIGHT * Y_LENGTH;

mod data;
mod game;
mod main_menu;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    InGame,
    Paused,
}
pub fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "test".to_string(),
            width: SCREEN_WIDTH as f32,
            height: SCREEN_HEIGHT as f32,
            resizable: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        // .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_state(AppState::MainMenu)
        .add_startup_system(setup.system()) // setup all
        .add_plugin(main_menu::MainMenuPlugin)
        .add_plugin(game::GamePlugin)
        // systems to run only in the main menu
        // .add_system_set(
        // SystemSet::on_update(AppState::MainMenu).with_system(main_menu::setup.system()),
        // )
        // setup when entering the state
        // cleanup when exiting the state
        // .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(text::setup.system()))
        // .add_system_set(
        // SystemSet::on_update(AppState::MainMenu).with_system(main_menu::text_update_system.system()),
        // )
        // .add_startup_system(text::setup.system())
        // .add_system(spawn_block_element.system())
        // .add_system(position_transform.system())
        // .add_system(text::text_update_system.system())
        // .add_system(text::text_color_system.system())
        .run();
}
