use crate::reference::Reference;

use super::Screen;

#[derive(Debug, PartialEq)]
pub struct Loading {}

impl Reference for Loading {
    fn process(_frame: &image::DynamicImage) -> Option<Screen> {
        Some(Screen::Loading(Loading {}))
    }

    fn compare(frame: &image::DynamicImage) -> bool {
        let crop = frame.crop_imm(630, 0, 20, 1);

        return crop.to_rgb16().pixels().all(|p| {
            return p.0.iter().all(|c| c > &61_000);
        });
    }
}
