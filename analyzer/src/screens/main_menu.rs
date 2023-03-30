use super::super::hasher;
use super::super::reference::Reference;
use super::Screen;
use crate::load_reference_hash;
use lazy_static::lazy_static;
use serde::Serialize;

lazy_static! {
    static ref REFERENCE_HASH: image_hasher::ImageHash = load_reference_hash!("main_menu.jpg");
}

#[derive(Debug, PartialEq, Serialize, Clone, Copy)]
pub struct MainMenu {}

impl Reference for MainMenu {
    fn process(_frame: &image::DynamicImage) -> Option<Screen> {
        Some(Screen::MainMenu(MainMenu {}))
    }

    fn compare(frame: &image::DynamicImage) -> bool {
        let crop = frame.crop_imm(220, 467, 122, 24);

        let check_hash = hasher::hash_image(crop);
        let delta = REFERENCE_HASH.dist(&check_hash);

        return delta <= 10;
    }
}
