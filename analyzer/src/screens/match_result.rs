use super::Screen;
use crate::color::average_colors;
use crate::color::max_color_diff;
use crate::hasher;
use crate::load_reference_hash;
use crate::reference::Reference;
use lazy_static::lazy_static;
use rayon::prelude::*;
use serde::{ser::SerializeMap, Serialize, Serializer};

lazy_static! {
    static ref REFERENCE_HASH_150: image_hasher::ImageHash =
        load_reference_hash!("match_result/150.jpg");
    static ref REFERENCE_HASH_200: image_hasher::ImageHash =
        load_reference_hash!("match_result/200.jpg");
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct PlayerResult {
    #[serde(skip_serializing)]
    index: u8,

    position: u8,

    #[serde(skip_serializing_if = "Option::is_none")]
    score: Option<u8>,
}

fn player_result_vec_serializer<S: Serializer>(
    player_results: &Vec<PlayerResult>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let mut map = serializer.serialize_map(Some(player_results.len()))?;

    for player in player_results.iter() {
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
pub struct MatchResult {
    #[serde(serialize_with = "player_result_vec_serializer", flatten)]
    players: Vec<PlayerResult>,
    speed: Option<u8>,
}

const POSITION_HEIGHT: u32 = 38;
const POSITION_MARGIN: u32 = 4;
const COLOR_STRIP_HEIGHT: usize = 3;
const COLOR_THRESHOLD: usize = 55_000;

impl Reference for MatchResult {
    fn compare(frame: &image::DynamicImage) -> bool {
        race_speed(frame).is_some()
    }

    fn process(frame: &image::DynamicImage) -> Option<Screen> {
        let mut players = (0..12)
            .par_bridge()
            .filter_map(|i| {
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

                let player = if average_red > COLOR_THRESHOLD && average_green > COLOR_THRESHOLD {
                    println!("yellow: {}", i + 1);
                    0
                } else if average_green > COLOR_THRESHOLD && average_blue > COLOR_THRESHOLD {
                    println!("blue: {}", i + 1);
                    1
                } else if average_green > COLOR_THRESHOLD {
                    println!("green: {}", i + 1);
                    3
                } else if average_red > COLOR_THRESHOLD {
                    println!("red: {}", i + 1);
                    2
                } else {
                    return None;
                };

                let mut score_section = frame.crop_imm(543, 132 + offset, 45, POSITION_HEIGHT);

                let score = calculate_score(&mut score_section);

                Some(PlayerResult {
                    index: player,
                    position: (i as u8) + 1,
                    score,
                })
            })
            .collect::<Vec<PlayerResult>>();
        players.sort_unstable_by(|a, b| a.index.cmp(&b.index));
        let result = MatchResult {
            players,
            speed: race_speed(frame),
        };
        return Some(Screen::MatchResult(result));
    }
}

fn race_speed(frame: &image::DynamicImage) -> Option<u8> {
    let crop = frame.crop_imm(37, 28, 99, 26);

    let check_hash = hasher::hash_image(crop);

    if REFERENCE_HASH_200.dist(&check_hash) < 10 {
        Some(200)
    } else if REFERENCE_HASH_150.dist(&check_hash) < 10 {
        Some(150)
    } else {
        None
    }
}

fn calculate_score(section: &mut image::DynamicImage) -> Option<u8> {
    max_color_diff(section, 130);

    let tens = get_number(&section, 0);
    let ones = get_number(&section, 23);

    tens.zip(ones).map(|(t, o)| (t * 10) + o)
}

fn get_number(section: &image::DynamicImage, x_offset: u32) -> Option<u8> {
    let top = is_black(&section.crop_imm(x_offset + 7, 6, 3, 1));
    let top_left = is_black(&section.crop_imm(x_offset + 2, 10, 1, 3));
    let top_right = is_black(&section.crop_imm(x_offset + 16, 10, 1, 3));
    let center = is_black(&section.crop_imm(x_offset + 7, 18, 3, 1));
    let bottom_left = is_black(&section.crop_imm(x_offset + 2, 22, 3, 1));
    let bottom_right = is_black(&section.crop_imm(x_offset + 16, 22, 3, 1));
    let bottom = is_black(&section.crop_imm(x_offset + 7, 30, 3, 1));

    match (
        top,
        top_left,
        top_right,
        center,
        bottom_left,
        bottom_right,
        bottom,
    ) {
        (false, false, true, false, false, true, false) => Some(1),
        (true, false, true, true, true, false, true) => Some(2),
        (true, false, true, true, false, true, true) => Some(8),
        (false, true, true, true, false, true, false) => Some(4),
        (true, true, false, true, false, true, true) => Some(5),
        (true, true, false, true, true, true, true) => Some(6),
        (true, false, true, false, false, true, false) => Some(7),
        (true, true, true, true, true, true, true) => Some(8),
        (true, true, true, true, false, true, true) => Some(9),
        _ => None,
    }
}

fn is_black(crop: &image::DynamicImage) -> bool {
    let [r, g, b] = average_colors(crop);
    let average = (r as u32 + g as u32 + b as u32) / 3;

    return average < 25_000;
}
