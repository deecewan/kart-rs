use super::Screen;
use crate::hasher;
use crate::load_reference_hash;
use crate::reference::Reference;
use lazy_static::lazy_static;
use serde::Serialize;

lazy_static! {
  static ref REFERENCE_HASH: image_hasher::ImageHash =
      load_reference_hash!("loading/loading_reference.jpg");
}

#[derive(Debug, PartialEq, Serialize, Clone, Copy)]
pub struct Loading {}

impl Reference for Loading {
    fn process(_frame: &image::DynamicImage) -> Option<Screen> {
        Some(Screen::Loading(Loading {}))
    }

    fn compare(frame: &image::DynamicImage) -> bool {
        let crop = frame.crop_imm(670, 20, 100, 100);
        let check_hash = hasher::hash_image(crop);
        let delta = REFERENCE_HASH.dist(&check_hash);
        return delta < 5;
    }
}
