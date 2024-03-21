use bevy::{
    prelude::*,
    window::{EnabledButtons, WindowMode, WindowResolution},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::Windowed,
                resolution: WindowResolution::new(800.0, 700.0),
                title: "Hampus Huledals Portfolio".to_string(),
                resizable: true,
                enabled_buttons: EnabledButtons {
                    maximize: true,
                    minimize: true,
                    close: true,
                },
                decorations: true,
                transparent: false,
                focused: true,
                visible: true,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("sprites/PNG/green_panel.png"),
        ..default()
    });
}
