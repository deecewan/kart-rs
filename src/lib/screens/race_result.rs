use super::Screen;
use crate::color::{
    average_colors, lightness, mostly_blue, mostly_green, mostly_red, COLOR_THRESHOLD,
};
use crate::reference::Reference;
use rayon::prelude::*;
use serde::{ser::SerializeMap, Serialize, Serializer};

const SCOREBOARD_TOP_MARGIN: u32 = 50;
const SCOREBOARD_PLAYER_HEIGHT: u32 = 48;
const SCOREBOARD_PLAYER_MARGIN: u32 = 4;

const POINTS_AWARDED: [u8; 12] = [15, 12, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1];

fn player_vec_serializer<S: Serializer>(
    players: &Vec<Player>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let mut map = serializer.serialize_map(Some(players.len()))?;

    for player in players.iter() {
        let name = match player.index {
            0 => "player_one",
            1 => "player_two",
            2 => "player_three",
            3 => "player_four",
            _ => panic!("too many players! only four supported!"),
        };

        map.serialize_entry(name, player)?;
    }

    map.end()
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct RaceResult {
    #[serde(serialize_with = "player_vec_serializer", flatten)]
    players: Vec<Player>,
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Player {
    #[serde(skip_serializing)]
    index: u8,
    position: u8,
    points: u8,
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

        let mut players: Vec<_> = (0..12)
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
                let points = POINTS_AWARDED[i as usize];
                if average_red > COLOR_THRESHOLD && average_green > COLOR_THRESHOLD {
                    Some(Player {
                        index: 0,
                        position,
                        points,
                    })
                } else if average_green > COLOR_THRESHOLD && average_blue > COLOR_THRESHOLD {
                    Some(Player {
                        index: 1,
                        position,
                        points,
                    })
                } else if average_green > COLOR_THRESHOLD {
                    Some(Player {
                        index: 3,
                        position,
                        points,
                    })
                } else if average_red > COLOR_THRESHOLD {
                    Some(Player {
                        index: 2,
                        position,
                        points,
                    })
                } else {
                    None
                }
            })
            .collect();

        players.sort_unstable_by(|a, b| a.index.cmp(&b.index));

        Some(Screen::RaceResult(RaceResult { players }))
    }
}
