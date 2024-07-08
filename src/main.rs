use bevy::{
    color::palettes::css::{MAROON, RED, YELLOW},
    prelude::*,
    text::TextLayoutInfo,
    ui::widget::TextFlags,
    window::WindowMode,
    winit::WinitSettings,
};

mod xml_texture_atlas;

#[derive(Debug, Component)]
struct RootNode;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::Windowed,
                    //resolution: WindowResolution::new(1920.0, 1080.0),
                    title: "Hampus Huledals Portfolio".to_string(),
                    resizable: true,
                    decorations: true,
                    transparent: false,
                    focused: true,
                    fit_canvas_to_parent: true,
                    ..Default::default()
                }),
                ..Default::default()
            })
            .set(AssetPlugin {
                mode: AssetMode::Unprocessed,
                ..Default::default()
            })
            .set(ImagePlugin::default_nearest()),))
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let header_font = asset_server.load::<Font>("./fonts/Ubuntu/Ubuntu-Bold.ttf");
    let body_font = asset_server.load::<Font>("./fonts/Ubuntu/Ubuntu-Regular.ttf");

    //root node
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    margin: UiRect::all(Val::Px(0.)),
                    align_self: AlignSelf::Stretch,
                    flex_wrap: FlexWrap::Wrap,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::FlexStart,
                    align_content: AlignContent::FlexStart,
                    ..default()
                },
                ..default()
            },
            Name::new("root"),
            RootNode,
        ))
        .with_children(|builder| {
            // Add header
            builder
                .spawn((
                    NodeBundle {
                        style: Style {
                            margin: UiRect::bottom(Val::Px(16.)),
                            align_self: AlignSelf::Stretch,
                            justify_content: JustifyContent::FlexStart,
                            align_items: AlignItems::Center,
                            align_content: AlignContent::FlexStart,
                            width: Val::Vw(100.),
                            height: Val::Vh(10.),
                            ..default()
                        },
                        background_color: Color::oklch(0.31, 0.01, 271.22).into(),
                        ..default()
                    },
                    Name::new("Header"),
                ))
                .with_children(|builder| {
                    builder.spawn((
                        TextBundle {
                            text: Text::from_section(
                                "Hampus Huledal",
                                TextStyle {
                                    font: header_font.clone(),
                                    font_size: 32.0,
                                    color: Color::WHITE,
                                },
                            ),
                            style: Style {
                                margin: UiRect::left(Val::Px(16.0)),
                                ..default()
                            },
                            ..default()
                        },
                        Name::new("Header Text"),
                    ));
                });

            // Add Body
            builder
                .spawn((
                    Name::new("Body"),
                    NodeBundle {
                        style: Style {
                            align_self: AlignSelf::Stretch,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            align_content: AlignContent::FlexStart,
                            flex_direction: FlexDirection::Column,
                            height: Val::Percent(100.0),
                            width: Val::Vw(100.0),
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|builder| {
                    let header_text = Text::from_sections([
                        TextSection {
                            value : "Welcome to my portfolio!".to_string(),
                            style : TextStyle {
                                font : header_font.clone(),
                                font_size : 32.0,
                                color: Color::srgba(1.0, 1.0, 1.0, 0.6)
                            }
                        },
                    ]).with_justify(JustifyText::Center);

                    builder.spawn(TextBundle {
                        text: header_text,
                        style: Style {
                            margin: UiRect::all(Val::Px(5.0)),
                            ..default()
                        },
                        ..default()
                    });

                    let body_text = Text::from_sections([
                        TextSection {
                            value : "It is made using bevy which is a game engine. You may wonder why i decided to use it? The answer is that i'm a game developer not a web developer and i wanted a project that both was useful and fun to do. Be aware that it is still very much under development".to_string(),
                            style : TextStyle {
                                font : body_font.clone(),
                                font_size : 16.0,
                                color: Color::srgba(1.0, 1.0, 1.0, 0.6)
                            }
                        },
                    ]).with_justify(JustifyText::Center);

                    builder.spawn(TextBundle {
                        text: body_text,
                        style: Style {
                            max_width: Val::Percent(50.0),
                            ..default()
                        },
                        ..default()
                    });
                });

                
        });
}
