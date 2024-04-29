use bevy::{
    app::{App, Plugin, Update},
    asset::{Asset, AssetApp, AssetLoader, Assets, AsyncReadExt, Handle},
    ecs::{
        component::Component,
        system::{Query, Res},
    },
    log::warn,
    math::{URect, UVec2},
    reflect::TypePath,
    render::texture::Image,
    sprite::{TextureAtlas, TextureAtlasLayout},
    utils::HashMap,
};
use thiserror::Error;
use xml::{reader::XmlEvent, EventReader};

pub struct XMLTextureAtlasPlugin;

impl Plugin for XMLTextureAtlasPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<XMLTextureAtlas>()
            .register_asset_loader(XMLTextureAtlasLoader)
            .add_systems(Update, update_atlas);
    }
}

#[derive(Debug, Component)]
pub struct XMLTextureAtlasComponent {
    pub name: String,
    pub atlas: Handle<XMLTextureAtlas>,
}

#[derive(Debug, Asset, TypePath)]
pub struct XMLTextureAtlas {
    //layout: Handle<TextureAtlasLayout>,
    names: HashMap<String, usize>,
}

#[derive(Default)]
struct XMLTextureAtlasLoader;

#[derive(Debug, Error)]
enum XMLTextureAtlasError {
    #[error("failed to read XML texture atlas")]
    Io(#[from] std::io::Error),
    #[error("failed to parse xml")]
    XML(#[from] xml::reader::Error),
    #[error("failed to parse from string to float")]
    ParseError(#[from] std::num::ParseIntError),
    #[error("{0} is missing atrribute {1}")]
    MissingAttribute(String, String),
    #[error("failed to discern the directory of the asset")]
    Path,
    #[error("failed to load image")]
    DirectLoatError(#[from] bevy::asset::LoadDirectError),
    #[error("failed to resolve path for the image")]
    AssetPathParseError(#[from] bevy::asset::ParseAssetPathError),
    #[error("failed to get loaded image")]
    ImageGetError,
    #[error("not a texture atlas in file")]
    NoTextureAtlas,
}

impl AssetLoader for XMLTextureAtlasLoader {
    type Asset = XMLTextureAtlas;
    type Settings = ();
    type Error = XMLTextureAtlasError;

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            let mut parser = EventReader::new(bytes.as_slice());

            let mut finished_asset = None;

            loop {
                let event = parser.next()?;

                match event {
                    XmlEvent::StartElement {
                        name,
                        attributes,
                        namespace: _,
                    } => {
                        if name.local_name == "TextureAtlas" {
                            let attributes: HashMap<String, String> = attributes
                                .into_iter()
                                .map(|a| (a.name.local_name, a.value))
                                .collect();

                            let image_path = attributes.get("imagePath").ok_or(
                                XMLTextureAtlasError::MissingAttribute(
                                    "TextureAtlas".to_string(),
                                    "imagePath".to_string(),
                                ),
                            )?;

                            let dir = load_context
                                .asset_path()
                                .parent()
                                .ok_or(XMLTextureAtlasError::Path)?;

                            let image_path = dir.resolve(&image_path)?;

                            let mut image_context = load_context.begin_labeled_asset();

                            let image = image_context
                                .load_direct(image_path.clone())
                                .await?
                                .take::<Image>()
                                .ok_or(XMLTextureAtlasError::ImageGetError)?;

                            let (layout, names) =
                                parse_texture_atlas(&mut parser, &name, &image.size())?;

                            load_context.add_loaded_labeled_asset(
                                "image".to_string(),
                                image_context.finish(image, None),
                            );

                            load_context.add_labeled_asset("layout".to_string(), layout);

                            finished_asset = Some(XMLTextureAtlas { names })
                        }
                    }
                    XmlEvent::EndDocument => break,
                    _ => (),
                }
            }

            finished_asset.ok_or(XMLTextureAtlasError::NoTextureAtlas)
        })
    }

    fn extensions(&self) -> &[&str] {
        &[".xml"]
    }
}

fn parse_texture_atlas(
    parser: &mut EventReader<&[u8]>,
    parent_name: &xml::name::OwnedName,
    sheet_size: &UVec2,
) -> Result<(TextureAtlasLayout, HashMap<String, usize>), XMLTextureAtlasError> {
    let mut layout = TextureAtlasLayout::new_empty(*sheet_size);
    let mut names = HashMap::new();

    'parse_loop: loop {
        let event = parser.next()?;

        match event {
            xml::reader::XmlEvent::EndDocument => break 'parse_loop,
            xml::reader::XmlEvent::StartElement {
                name,
                attributes,
                namespace: _,
            } => {
                if name.local_name == "SubTexture" {
                    let attributes: HashMap<String, String> = attributes
                        .into_iter()
                        .map(|a| (a.name.local_name, a.value))
                        .collect();

                    let name =
                        attributes
                            .get("name")
                            .ok_or(XMLTextureAtlasError::MissingAttribute(
                                "SubTexture".to_string(),
                                "Name".to_string(),
                            ))?;

                    let origin = UVec2::new(
                        attributes
                            .get("x")
                            .ok_or(XMLTextureAtlasError::MissingAttribute(
                                "SubTexture".to_string(),
                                "x".to_string(),
                            ))?
                            .parse()?,
                        attributes
                            .get("y")
                            .ok_or(XMLTextureAtlasError::MissingAttribute(
                                "SubTexture".to_string(),
                                "y".to_string(),
                            ))?
                            .parse()?,
                    );

                    let size = UVec2::new(
                        attributes
                            .get("width")
                            .ok_or(XMLTextureAtlasError::MissingAttribute(
                                "SubTexture".to_string(),
                                "width".to_string(),
                            ))?
                            .parse::<u32>()?,
                        attributes
                            .get("height")
                            .ok_or(XMLTextureAtlasError::MissingAttribute(
                                "SubTexture".to_string(),
                                "height".to_string(),
                            ))?
                            .parse::<u32>()?,
                    );

                    let rect = URect::from_corners(origin, origin + size);

                    names.insert(name.clone(), layout.add_texture(rect));
                } else {
                    warn!("Unexpected element {name} when parsing texture atlas")
                }
            }
            xml::reader::XmlEvent::EndElement { name } => {
                if &name == parent_name {
                    break 'parse_loop;
                }
            }
            _ => {}
        }
    }

    Ok((layout, names))
}

fn update_atlas(
    atlas: Res<Assets<XMLTextureAtlas>>,
    mut query: Query<(&XMLTextureAtlasComponent, &mut TextureAtlas)>,
) {
    for (xml_atlas, mut texture_atlas) in &mut query {
        if let Some(atlas) = atlas.get(&xml_atlas.atlas) {
            match atlas.names.get(&xml_atlas.name) {
                Some(index) => texture_atlas.index = *index,
                None => warn!("invalid sprite sheeet texture name: {}", xml_atlas.name),
            }
        }
    }
}
