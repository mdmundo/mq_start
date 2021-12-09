use macroquad::prelude::*;

#[macroquad::main("Bandeira do Japão")]
async fn main() {
    loop {
        clear_background(WHITE);

        draw_circle(
            screen_width() / 2.0,
            screen_height() / 2.0,
            screen_height() / 4.0,
            RED,
        );

        next_frame().await
    }
}
