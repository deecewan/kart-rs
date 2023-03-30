use super::screens::Screen;

pub trait Reference {
    fn process(frame: &image::DynamicImage) -> Option<Screen>;
    fn compare(frame: &image::DynamicImage) -> bool;
}
