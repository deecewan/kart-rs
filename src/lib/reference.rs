use super::{color::lightness, screens::Screen};
use crate::color::average_colors;

pub trait Reference {
    fn process(frame: &image::DynamicImage) -> Option<Screen>;
    fn compare(frame: &image::DynamicImage) -> bool;

    fn is_splitscreen(frame: &image::DynamicImage) -> bool {
        let width = frame.width();
        let height = frame.height();

        let top = get_overall_average(&frame.crop_imm((width / 2) - 1, 2, 2, 4));
        let left = get_overall_average(&frame.crop_imm(2, (height / 2) - 1, 4, 2));
        let right = get_overall_average(&frame.crop_imm(width - 4, (height / 2) - 1, 4, 2));
        let bottom = get_overall_average(&frame.crop_imm((width / 2) - 1, height - 4, 2, 4));

        let rgb = frame.to_rgb16();
        let top_left = lightness(rgb.get_pixel(0, 0));
        let bottom_left = lightness(rgb.get_pixel(0, height - 1));

        return top <= 5000
            && bottom <= 5000
            && left <= 5000
            && right <= 5000
            && (top_left > 3800 || bottom_left > 3800);
    }
}

fn get_overall_average(image: &image::DynamicImage) -> u32 {
    let [r, g, b] = average_colors(&image);

    let total = r as u32 + g as u32 + b as u32;

    return total / 3;
}
