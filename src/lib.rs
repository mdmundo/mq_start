use macroquad::miniquad::conf::Icon;

pub fn default_logo() -> Icon {
    // https://developer.mozilla.org/en-US/docs/Web/API/ImageData/data
    // https://docs.rs/miniquad/0.3.0-alpha.41/src/miniquad/conf.rs.html
    let mut i = 0;
    let mut small = [0_u8; 1024];
    let mut medium = [0_u8; 4096];
    let mut big = [0_u8; 16384];

    while i < 1024 {
        let x = (i % 64) as f64 / 64.0 * 255.0;
        let y = (i as f64 / 64.0).ceil() / 100.0 * 255.0;

        small[i + 0] = x as u8;
        small[i + 1] = y as u8;
        small[i + 2] = 255_u8 - x as u8;
        small[i + 3] = 255_u8;

        i += 4;
    }
    i = 0;
    while i < 4096 {
        let x = (i % 128) as f64 / 128.0 * 255.0;
        let y = (i as f64 / 128.0).ceil() / 100.0 * 255.0;

        medium[i + 0] = x as u8;
        medium[i + 1] = y as u8;
        medium[i + 2] = 255_u8 - x as u8;
        medium[i + 3] = 255_u8;

        i += 4;
    }
    i = 0;
    while i < 16384 {
        let x = (i % 256) as f64 / 256.0 * 255.0;
        let y = (i as f64 / 256.0).ceil() / 100.0 * 255.0;

        big[i + 0] = x as u8;
        big[i + 1] = y as u8;
        big[i + 2] = 255_u8 - x as u8;
        big[i + 3] = 255_u8;

        i += 4;
    }

    Icon { small, medium, big }
}
