use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Bandeira do Brasil".to_owned(),
        fullscreen: true,
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
        clear_background(DARKGREEN);

        draw_triangle(
            Vec2::new(screen_width() / 8.0, screen_height() / 2.0),
            Vec2::new(screen_width() / 2.0, screen_height() / 8.0),
            Vec2::new(
                screen_width() - (screen_width() / 8.0),
                screen_height() / 2.0,
            ),
            YELLOW,
        );
        draw_triangle(
            Vec2::new(screen_width() / 8.0, screen_height() / 2.0),
            Vec2::new(
                screen_width() / 2.0,
                screen_height() - (screen_height() / 8.0),
            ),
            Vec2::new(
                screen_width() - (screen_width() / 8.0),
                screen_height() / 2.0,
            ),
            YELLOW,
        );
        draw_circle(
            screen_width() / 2.0,
            screen_height() / 2.0,
            smallest_screen() / 4.0,
            BLUE,
        );

        next_frame().await
    }
}
