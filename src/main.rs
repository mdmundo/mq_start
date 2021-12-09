use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Icon".to_owned(),
        icon: Some(mq_start::default_logo()),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
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
