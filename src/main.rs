use bevy::{
    prelude::*,
    window::{WindowMode, WindowResolution},
};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        mode: WindowMode::Windowed,
                        resolution: WindowResolution::new(800.0, 700.0),
                        title: "Hampus Huledals Portfolio".to_string(),
                        resizable: true,
                        decorations: true,
                        transparent: false,
                        focused: true,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(AssetPlugin {
                    mode: AssetMode::Processed,
                    ..Default::default()
                }),
        )
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    //root node
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(500.0),
                        height: Val::Px(500.0),
                        margin: UiRect::all(Val::Px(1.0)),
                        ..Default::default()
                    },

                    background_color: Color::WHITE.into(),
                    ..Default::default()
                },
                UiImage::new(asset_server.load("sprites/blue_panel.png")),
                ImageScaleMode::Sliced(TextureSlicer {
                    border: BorderRect::square(10.0),
                    max_corner_scale: 1.0,
                    ..Default::default()
                }),
            ));
        });
}
