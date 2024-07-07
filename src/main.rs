use bevy::{
    prelude::*,
    window::{WindowMode, WindowResized},
    winit::WinitSettings,
};
use xml_texture_atlas::{XMLTextureAtlas, XMLTextureAtlasComponent, XMLTextureAtlasPlugin};

mod xml_texture_atlas;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        mode: WindowMode::Fullscreen,
                        //resolution: WindowResolution::new(1920.0, 1080.0),
                        title: "Hampus Huledals Portfolio".to_string(),
                        resizable: true,
                        decorations: true,
                        transparent: false,
                        focused: true,
                        fit_canvas_to_parent:true,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(AssetPlugin {
                    mode: AssetMode::Unprocessed,
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
            XMLTextureAtlasPlugin,
        ))
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let text_style = TextStyle {
        font_size: 20.,
        ..default()
    };

    let xml_atlas: Handle<XMLTextureAtlas> = asset_server.load("spritesheets/blueSheet.xml");
    let texture_handle: Handle<Image> = asset_server.load("spritesheets/blueSheet.xml#image");
    let texture_atlas_handle: Handle<TextureAtlasLayout> =
        asset_server.load("spritesheets/blueSheet.xml#layout");

    //root node
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(text_style.font_size * 2.),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                ImageBundle {
                    style: Style {
                        width: Val::Px(190.),
                        height: Val::Px(49.),
                        ..default()
                    },
                    image: UiImage::new(texture_handle),
                    ..Default::default()
                },
                TextureAtlas::from(texture_atlas_handle) ,
                XMLTextureAtlasComponent {
                    name: "blue_button00.png".to_string(),
                    atlas: xml_atlas,
                },
            ));
        });
}

