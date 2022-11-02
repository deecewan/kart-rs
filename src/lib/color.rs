use image::Rgb;

pub fn lightness(pixel: &Rgb<u16>) -> u16 {
    let mx = max(pixel.0);
    let mn = min(pixel.0);

    ((mx as u32 + mn as u32) / 2) as u16
}

fn max([red, green, blue]: [u16; 3]) -> u16 {
    return red.max(green).max(blue);
}

fn min([red, green, blue]: [u16; 3]) -> u16 {
    return red.min(green).min(blue);
}

pub fn average_colors(im: &image::DynamicImage, size: usize) -> [u16; 3] {
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
