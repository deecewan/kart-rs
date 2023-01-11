use super::Screen;
use crate::color::{
    average_colors, lightness, mostly_blue, mostly_green, mostly_red, COLOR_THRESHOLD,
};
use crate::reference::Reference;
use rayon::prelude::*;

const SCOREBOARD_TOP_MARGIN: u32 = 50;
const SCOREBOARD_PLAYER_HEIGHT: u32 = 48;
const SCOREBOARD_PLAYER_MARGIN: u32 = 4;

#[derive(Debug, PartialEq)]
pub struct RaceResult {
    players: Vec<Player>,
}

#[derive(Debug, PartialEq)]
pub struct Player {
    index: u8,
    position: u8,
}

impl Reference for RaceResult {
    fn compare(frame: &image::DynamicImage) -> bool {
        if !Self::is_splitscreen(frame) {
            return false;
        }
        let width = frame.width();
        let frame = frame.to_rgb16();
        let pixel = frame.get_pixel(width / 2 - 1, 50);

        let value = lightness(pixel);

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
            return None;
        }

        let players: Vec<_> = (0..12)
            .par_bridge()
            .filter_map(|i| {
                let offset = 2 + i * (SCOREBOARD_PLAYER_HEIGHT + SCOREBOARD_PLAYER_MARGIN);
                let check_slice = frame.crop_imm(
                    (frame.width() / 2) - 1,
                    SCOREBOARD_TOP_MARGIN + offset,
                    2,
                    2,
                );

                let [average_red, average_green, average_blue] = average_colors(&check_slice);

                let position: u8 = i as u8 + 1;
                if average_red > COLOR_THRESHOLD && average_green > COLOR_THRESHOLD {
                    Some(Player { index: 0, position })
                } else if average_green > COLOR_THRESHOLD && average_blue > COLOR_THRESHOLD {
                    Some(Player { index: 1, position })
                } else if average_green > COLOR_THRESHOLD {
                    Some(Player { index: 3, position })
                } else if average_red > COLOR_THRESHOLD {
                    Some(Player { index: 2, position })
                } else {
                    None
                }
            })
            .collect();

        Some(Screen::RaceResult(RaceResult { players }))
    }
}
