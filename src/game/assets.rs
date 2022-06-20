pub use amethyst::{
    prelude::*,
    assets::{AssetStorage, Loader, Handle},
    renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
};

pub fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();

        loader.load(
            "textures/sprite_sheet.png",
            ImageFormat::default(),
            (),
            &texture_storage
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();

    loader.load(
        "textures/sprite_sheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_storage
    )
}