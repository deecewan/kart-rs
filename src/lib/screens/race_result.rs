use super::Screen;
use crate::color::{average_colors, lightness, mostly_blue, mostly_green, mostly_red};
use crate::reference::Reference;
use rayon::prelude::*;

const SCOREBOARD_TOP_MARGIN: u32 = 50;
const SCOREBOARD_PLAYER_HEIGHT: u32 = 48;
const SCOREBOARD_PLAYER_MARGIN: u32 = 4;
const COLOR_THRESHOLD: u16 = 45_000;

#[derive(Debug, PartialEq)]
pub struct RaceResult {}

impl Reference for RaceResult {
    fn compare(frame: &image::DynamicImage) -> bool {
        if !Self::is_splitscreen(frame) {
            return false;
        }
        let width = frame.width();
        let frame = frame.to_rgb16();
        let pixel = frame.get_pixel(width / 2 - 1, 50);

        let value = lightness(pixel);

        println!("result: {:?}", value);

        return value > 12_850;
    }

    fn process(frame: &image::DynamicImage) -> Option<Screen> {
        let pixels = frame
            .crop_imm(320, 70, 1, 600)
            .to_rgb16()
            .pixels()
            .filter(|p| mostly_red(p.0) || mostly_blue(p.0) || mostly_green(p.0))
            .count();

        if pixels > 10 {
            println!("with moves");
            return None;
        }

        (0..12).par_bridge().for_each(|i| {
            let offset = 2 + i * (SCOREBOARD_PLAYER_HEIGHT + SCOREBOARD_PLAYER_MARGIN);
            let check_slice = frame.crop_imm(
                (frame.width() / 2) - 1,
                SCOREBOARD_TOP_MARGIN + offset,
                2,
                2,
            );

            let [average_red, average_green, average_blue] = average_colors(&check_slice, 4);

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

        Some(Screen::RaceResult(RaceResult {}))
    }
}
