use crate::game::{Paddle, Side, ARENA_HEIGHT, PADDLE_HEIGHT};
use amethyst::{
    ecs::{Join, Read, ReadStorage, WriteStorage, System, SystemData},
    input::{InputHandler, StringBindings},
    core::Transform,
    derive::SystemDesc
};

#[derive(SystemDesc)]
pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<StringBindings>>
    );

    fn run(&mut self, (mut transforms, paddles, input): Self::SystemData) {
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            let movement = match paddle.side {
                Side::Left => input.axis_value("left_paddle"),
                Side::Right => input.axis_value("right_paddle")
            }.unwrap_or_default();

            let paddle_y = transform.translation().y;
            transform.set_translation_y(
                (paddle_y + movement)
                    .min(ARENA_HEIGHT - PADDLE_HEIGHT * 0.5)
                    .max(PADDLE_HEIGHT * 0.5)
            );
        }
    }
}
