use super::super::hasher;
use super::super::reference::Reference;
use crate::load_reference_hash;
use lazy_static::lazy_static;
use serde::Serialize;

lazy_static! {
    static ref REFERENCE_HASH: image_hasher::ImageHash =
        load_reference_hash!("select_character.jpg");
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct SelectCharacter {}

impl Reference for SelectCharacter {
    fn process(_frame: &image::DynamicImage) -> Option<super::Screen> {
        Some(super::Screen::SelectCharacter(SelectCharacter {}))
    }
    fn compare(frame: &image::DynamicImage) -> bool {
        let crop = frame.crop_imm(735, 435, 40, 35);

        let check_hash = hasher::hash_image(crop);
        let delta = REFERENCE_HASH.dist(&check_hash);

        return delta <= 10;
    }
}
