use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Icon".to_owned(),
        icon: Some(mq_start::default_logo()),
        ..Default::default()
    }
}

fn smallest_screen() -> f32 {
    let height = screen_height();
    let width = screen_width();
    if height < width {
        height
    } else {
        width
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(WHITE);

        // draw_triangle(v1, v2, v3, color)
        draw_circle(
            screen_width() / 2.0,
            screen_height() / 2.0,
            smallest_screen() / 4.0,
            RED,
        );

        next_frame().await
    }
}
