use super::{color::lightness, screens::Screen};

pub trait Reference {
    fn process(frame: &image::DynamicImage) -> Option<Screen>;
    fn compare(frame: &image::DynamicImage) -> bool;

    fn is_splitscreen(frame: &image::DynamicImage) -> bool {
        let frame = frame.to_rgb16();
        let width = frame.width();
        let height = frame.height();
        let top = lightness(frame.get_pixel((width / 2) - 1, 0));
        let left = lightness(frame.get_pixel(0, (height / 2) - 1));
        let right = lightness(frame.get_pixel(width - 1, (height / 2) - 1));
        let bottom = lightness(frame.get_pixel((width / 2) - 1, height - 1));
        let top_left = lightness(frame.get_pixel(0, 0));
        let bottom_left = lightness(frame.get_pixel(0, height - 1));

        return top <= 5000
            && bottom <= 5000
            && left <= 5000
            && right <= 5000
            && (top_left > 3800 || bottom_left > 3800);
    }
}
