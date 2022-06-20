pub mod game;

use game::{Pong, systems::*};
use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle
    },
    core::TransformBundle,
    input::{InputBundle, StringBindings},
    utils::application_root_dir
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let binding_config_path = app_root.join("config").join("binding.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderingBundle::<DefaultBackend>::new()
            .with_plugin(RenderToWindow::from_config_path(display_config_path)?
                .with_clear([0, 0, 0, 1])
            )
            .with_plugin(RenderFlat2D::default())
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<StringBindings>::new()
            .with_bindings_from_file(binding_config_path)?
        )?
        .with(PaddleSystem, "paddle_system", &["input_system"]);

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, Pong, game_data)?;

    game.run();

    Ok(())
}
