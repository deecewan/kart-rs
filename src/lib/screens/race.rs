use std::fmt::Display;
use std::vec::Vec;

use super::Screen;
use crate::color::lightness;
use crate::hasher;
use crate::load_reference_hash;
use crate::reference::Reference;
use lazy_static::lazy_static;
use rayon::prelude::*;

struct ImageReference {
    files: Vec<image_hasher::ImageHash>,
}

lazy_static! {
    static ref GO_REFERENCE: image_hasher::ImageHash = load_reference_hash!("race/go.jpg");
    static ref FINISH_REFERENCE: image_hasher::ImageHash =
        load_reference_hash!("race/finished.jpg");
    static ref REFERENCE_HASHES: Vec<ImageReference> = vec![
        ImageReference {
            files: vec![
                load_reference_hash!("race/pos1.png"),
                load_reference_hash!("race/pos1-alt.png"),
            ],
        },
        ImageReference {
            files: vec![load_reference_hash!("race/pos2.png"),],
        },
        ImageReference {
            files: vec![load_reference_hash!("race/pos3.png"),],
        },
        ImageReference {
            files: vec![
                load_reference_hash!("race/pos4.png"),
                load_reference_hash!("race/pos4-alt.png"),
            ],
        },
        ImageReference {
            files: vec![load_reference_hash!("race/pos5.png"),],
        },
        ImageReference {
            files: vec![
                load_reference_hash!("race/pos6.png"),
                load_reference_hash!("race/pos6-alt.png"),
            ],
        },
        ImageReference {
            files: vec![
                load_reference_hash!("race/pos7.png"),
                load_reference_hash!("race/pos7-alt.png"),
            ],
        },
        ImageReference {
            files: vec![load_reference_hash!("race/pos8.png"),],
        },
        ImageReference {
            files: vec![
                load_reference_hash!("race/pos9.png"),
                load_reference_hash!("race/pos9-alt.png"),
            ],
        },
        ImageReference {
            files: vec![
                load_reference_hash!("race/pos10.png"),
                load_reference_hash!("race/pos10-alt.png"),
            ],
        },
        ImageReference {
            files: vec![
                load_reference_hash!("race/pos11.png"),
                load_reference_hash!("race/pos11-alt.png"),
            ],
        },
        ImageReference {
            files: vec![
                load_reference_hash!("race/pos12.png"),
                load_reference_hash!("race/pos12-alt.jpg"),
            ],
        },
    ];
}

#[derive(Debug, PartialEq)]
pub struct Race {
    pub players: Vec<Player>,
    pub starting: bool,
}

impl Display for Race {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = self.players.iter().fold(Ok(()), |res, player| {
            res.and(writeln!(
                f,
                "Player {}: {} ({:?})",
                player.index,
                player.position.unwrap_or(0),
                player.status,
            ))
        });

        if self.starting {
            res.and(writeln!(f, "(Race Starting!)"))
        } else {
            res
        }
    }
}

const POSITION_CROP: [[u32; 2]; 4] = [[57, 239], [1167, 239], [57, 599], [1167, 599]];
const FINISH_CROP: [[u32; 2]; 4] = [[159, 127], [799, 127], [159, 487], [799, 487]];

#[derive(Debug, PartialEq)]
pub enum Status {
    Racing,
    Finished,
}

#[derive(Debug, PartialEq)]
pub struct Player {
    index: u8,
    position: Option<u8>,
    status: Status,
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Player #{}: {:?} {:?}",
            self.index + 1,
            self.position,
            self.status
        )
    }
}

impl Reference for Race {
    fn process(frame: &image::DynamicImage) -> Option<Screen> {
        let mut players: Vec<Player> = (0..4)
            .par_bridge()
            .map(|p| {
                let position = get_position(&frame, p);

                let status = get_status(&frame, p);

                Player {
                    index: p as u8,
                    position,
                    status,
                }
            })
            .collect();

        players.sort_by(|a, b| a.index.cmp(&b.index));

        let starting = check_starting(&frame);

        Some(Screen::Race(Race { players, starting }))
    }

    fn compare(frame: &image::DynamicImage) -> bool {
        if !Self::is_splitscreen(frame) {
            return false;
        }

        let width = frame.width();
        let max_lightness = frame
            .crop_imm(width / 2 - 1, 48, 1, 8)
            .to_rgb16()
            .pixels()
            .map(|p| lightness(p))
            .max();

        match max_lightness {
            Some(max_lightness) => max_lightness < 5250,
            None => false,
        }
    }
}

fn check_starting(frame: &image::DynamicImage) -> bool {
    let image = frame.crop_imm(573, 292, 39, 37);
    let hash = hasher::hash_image(image);

    println!("dist: {}", GO_REFERENCE.dist(&hash));

    GO_REFERENCE.dist(&hash) < 15
}

fn get_position(frame: &image::DynamicImage, index: usize) -> Option<u8> {
    let [x, y] = POSITION_CROP[index];

    let image = frame.crop_imm(x, y, 36, 54).grayscale();
    let res = hasher::hash_image(image);

    REFERENCE_HASHES
        .par_iter()
        .enumerate()
        .find_any(|(_, hash)| hash.files.iter().any(|f| f.dist(&res) < 16))
        .map(|(i, _)| (i as u8) + 1)
}

fn get_status(frame: &image::DynamicImage, index: usize) -> Status {
    let [x, y] = FINISH_CROP[index];

    let image = frame.crop_imm(x, y, 38, 38);

    let res = hasher::hash_image(image);

    if FINISH_REFERENCE.dist(&res) < 15 {
        Status::Finished
    } else {
        Status::Racing
    }
}
