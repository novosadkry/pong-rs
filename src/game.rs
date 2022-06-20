pub mod assets;
pub mod systems;

pub use self::assets::*;
pub use self::systems::*;

pub use amethyst::{
    prelude::*,
    core::transform::Transform,
    ecs::{Component, DenseVecStorage},
    renderer::{Camera, SpriteSheet, SpriteRender},
};

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right
}

pub struct Pong;

pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Paddle {
    fn new(side: Side) -> Paddle {
        Paddle {
            side,
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
        }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<Paddle>();

        let sprite_sheet_handle = load_sprite_sheet(world);

        initialize_camera(world);
        initialize_paddles(world, sprite_sheet_handle);
    }
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

fn initialize_paddles(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let y = ARENA_HEIGHT * 0.5;

    let mut transform = Transform::default();
    transform.set_translation_xyz(PADDLE_WIDTH * 0.5, y, 0.0);

    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);

    world
        .create_entity()
        .with(Paddle::new(Side::Left))
        .with(sprite_render.clone())
        .with(transform)
        .build();

    transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH - PADDLE_WIDTH * 0.5, y, 0.0);

    world
        .create_entity()
        .with(Paddle::new(Side::Right))
        .with(sprite_render)
        .with(transform)
        .build();
}
