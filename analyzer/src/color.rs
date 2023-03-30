use image::GenericImage;
use image::GenericImageView;
use image::Rgb;

pub const COLOR_THRESHOLD: u16 = 45_000;

pub fn lightness(pixel: &Rgb<u16>) -> u16 {
    let mx = max(pixel.0);
    let mn = min(pixel.0);

    let total = mx as f32 + mn as f32;
    let percent = total / (2.0 * (u16::MAX as f32));
    let res = percent * (u16::MAX as f32);

    res as u16
}

fn max([red, green, blue]: [u16; 3]) -> u16 {
    return red.max(green).max(blue);
}

fn min([red, green, blue]: [u16; 3]) -> u16 {
    return red.min(green).min(blue);
}

pub fn average_colors(im: &image::DynamicImage) -> [u16; 3] {
    let size = (im.width() * im.height()) as usize;
    im.to_rgb16()
        .pixels()
        .map(|p| [p.0[0] as usize, p.0[1] as usize, p.0[2] as usize])
        .reduce(|a, p| [a[0] + p[0], a[1] + p[1], a[2] + p[2]])
        .map(|p| [p[0] / size, p[1] / size, p[2] / size])
        .map(|p| [p[0] as u16, p[1] as u16, p[2] as u16])
        .expect("Couldn't find the totals")
}

pub fn mostly_red([r, g, b]: [u16; 3]) -> bool {
    return r > 40_000 && g < 20_000 && b < 20_000;
}

pub fn mostly_green([r, g, b]: [u16; 3]) -> bool {
    return g > 40_000 && r < 20_000 && b < 20_000;
}

pub fn mostly_blue([r, g, b]: [u16; 3]) -> bool {
    return b > 40_000 && r < 20_000 && g < 20_000;
}

pub fn max_color_diff(image: &mut image::DynamicImage, cutoff: u16) {
    for y in 0..image.height() {
        for x in 0..image.width() {
            let px = image.get_pixel(x, y);
            let [r, g, b, _] = px.0;
            let avg = (r as u16 + g as u16 + b as u16) / 3;

            if avg > cutoff {
                image.put_pixel(x, y, image::Rgba::from([255, 255, 255, 1]));
            } else {
                image.put_pixel(x, y, image::Rgba::from([0, 0, 0, 1]));
            }
        }
    }
}

pub fn get_overall_average(image: &image::DynamicImage) -> u32 {
    let [r, g, b] = average_colors(&image);

    let total = r as u32 + g as u32 + b as u32;

    return total / 3;
}
