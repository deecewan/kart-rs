use super::Screen;
use crate::hasher;
use crate::load_reference_hash;
use crate::reference::Reference;
use lazy_static::lazy_static;
use rayon::prelude::*;

lazy_static! {
    static ref REFERENCE_HASH_150: image_hasher::ImageHash =
        load_reference_hash!("match_result/150.jpg");
    static ref REFERENCE_HASH_200: image_hasher::ImageHash =
        load_reference_hash!("match_result/150.jpg");
}

#[derive(Debug, PartialEq)]
pub struct MatchResult {}

const POSITION_HEIGHT: u32 = 38;
const POSITION_MARGIN: u32 = 4;
const COLOR_STRIP_HEIGHT: usize = 3;
const COLOR_THRESHOLD: usize = 55_000;

impl Reference for MatchResult {
    fn compare(frame: &image::DynamicImage) -> bool {
        let crop = frame.crop_imm(37, 28, 99, 26);

        let check_hash = hasher::hash_image(crop);

        return REFERENCE_HASH_200.dist(&check_hash) < 10
            || REFERENCE_HASH_150.dist(&check_hash) < 10;
    }

    fn process(frame: &image::DynamicImage) -> Option<Screen> {
        (0..12).par_bridge().for_each(|i| {
            let offset = i * (POSITION_HEIGHT + POSITION_MARGIN);
            let im = frame.crop_imm(104, 132 + offset, 1, COLOR_STRIP_HEIGHT as u32);

            let totals = im
                .to_rgb16()
                .pixels()
                .map(|p| [p.0[0] as usize, p.0[1] as usize, p.0[2] as usize])
                .reduce(|a, p| [a[0] + p[0], a[1] + p[1], a[2] + p[2]])
                .expect("Couldn't find the totals");
            let average_red = totals[0] / COLOR_STRIP_HEIGHT;
            let average_green = totals[1] / COLOR_STRIP_HEIGHT;
            let average_blue = totals[2] / COLOR_STRIP_HEIGHT;

            if average_red > COLOR_THRESHOLD && average_green > COLOR_THRESHOLD {
                println!("yellow: {}", i + 1);
            } else if average_green > COLOR_THRESHOLD && average_blue > COLOR_THRESHOLD {
                println!("blue: {}", i + 1);
            } else if average_green > COLOR_THRESHOLD {
                println!("green: {}", i + 1);
            } else if average_red > COLOR_THRESHOLD {
                println!("red: {}", i + 1);
            }
        });
        let result = MatchResult {};
        return Some(Screen::MatchResult(result));
    }
}
