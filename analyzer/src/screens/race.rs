use super::Screen;
use crate::color::{average_colors, COLOR_THRESHOLD};
use crate::hasher;
use crate::load_reference_hash;
use crate::reference::Reference;
use crate::util::is_splitscreen;
use lazy_static::lazy_static;
use rayon::prelude::*;
use serde::{ser::SerializeMap, Serialize, Serializer};
use std::fmt::Display;
use std::vec::Vec;

#[derive(Debug)]
struct ImageReference {
    files: Vec<image_hasher::ImageHash>,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Item {
    Banana,
    BananaDouble,
    BananaTriple,
    BlueShell,
    Bomb,
    Boomerang,
    Bullet,
    Coin,
    CrazyEight,
    FireFlower,
    Ghost,
    GoldenMushroom,
    GreenShell,
    GreenShellDouble,
    GreenShellTriple,
    Horn,
    Lightning,
    Mushroom,
    MushroomDouble,
    MushroomTriple,

    #[serde(rename = "pirhana-plant")]
    PiranhaPlant,

    RedShell,
    RedShellDouble,
    RedShellTriple,
    Squid,
    Star,
}

struct ItemReference {
    files: Vec<image_hasher::ImageHash>,
    item: Item,
    threshold: u8,
}

lazy_static! {
    static ref LAP_FLAG_REFERENCE: image_hasher::ImageHash =
        load_reference_hash!("race/lap_flag.jpg");
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
            files: vec![
                load_reference_hash!("race/pos5.png"),
                load_reference_hash!("race/pos5-alt.png"),
            ],
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
    static ref ITEM_HASHES: Vec<ItemReference> = vec![
        ItemReference {
            item: Item::BananaDouble,
            files: vec![
                load_reference_hash!("items/banana-double.jpg"),
                load_reference_hash!("items/banana-double_1.jpg"),
                load_reference_hash!("items/banana-double_2.jpg"),
                load_reference_hash!("items/banana-double_3.jpg"),
            ],
            threshold: 12,
        },
        ItemReference {
            item: Item::BananaTriple,
            files: vec![
                load_reference_hash!("items/banana-triple.jpg"),
                load_reference_hash!("items/banana-triple_1.jpg"),
                load_reference_hash!("items/banana-triple_2.jpg"),
                load_reference_hash!("items/banana-triple_3.jpg"),
            ],
            threshold: 12,
        },
        ItemReference {
            item: Item::Banana,
            files: vec![
                load_reference_hash!("items/banana.jpg"),
                load_reference_hash!("items/banana_1.jpg"),
                load_reference_hash!("items/banana_2.jpg"),
                load_reference_hash!("items/banana_3.jpg"),
                load_reference_hash!("items/banana_4.jpg"),
                load_reference_hash!("items/banana_5.jpg"),
            ],
            threshold: 14,
        },
        ItemReference {
            item: Item::BlueShell,
            files: vec![
                load_reference_hash!("items/blue-shell.jpg"),
                load_reference_hash!("items/blue-shell_1.jpg"),
            ],
            threshold: 12,
        },
        ItemReference {
            item: Item::Bomb,
            files: vec![
                load_reference_hash!("items/bomb.jpg"),
                load_reference_hash!("items/bomb_1.jpg"),
                load_reference_hash!("items/bomb_2.jpg"),
            ],
            threshold: 12,
        },
        ItemReference {
            item: Item::Boomerang,
            files: vec![
                load_reference_hash!("items/boomerang.jpg"),
                load_reference_hash!("items/boomerang_1.jpg"),
                load_reference_hash!("items/boomerang_2.jpg"),
                load_reference_hash!("items/boomerang_3.jpg"),
                load_reference_hash!("items/boomerang_4.jpg"),
            ],
            threshold: 12,
        },
        ItemReference {
            item: Item::Bullet,
            files: vec![
                load_reference_hash!("items/bullet.jpg"),
                load_reference_hash!("items/bullet_1.jpg"),
                load_reference_hash!("items/bullet_2.jpg"),
                load_reference_hash!("items/bullet_3.jpg"),
                load_reference_hash!("items/bullet_4.jpg"),
            ],
            threshold: 15,
        },
        ItemReference {
            item: Item::Coin,
            files: vec![
                load_reference_hash!("items/coin.jpg"),
                load_reference_hash!("items/coin_1.jpg"),
                load_reference_hash!("items/coin_3.jpg"),
            ],
            threshold: 15,
        },
        ItemReference {
            item: Item::CrazyEight,
            files: vec![load_reference_hash!("items/crazy-eight_meh.jpg"),],
            threshold: 16,
        },
        ItemReference {
            item: Item::FireFlower,
            files: vec![
                load_reference_hash!("items/fire-flower.jpg"),
                load_reference_hash!("items/fire-flower_1.jpg"),
                load_reference_hash!("items/fire-flower_2.jpg"),
                load_reference_hash!("items/fire-flower_3.jpg"),
                load_reference_hash!("items/fire-flower_use.jpg"),
            ],
            threshold: 12,
        },
        ItemReference {
            item: Item::Ghost,
            files: vec![
                load_reference_hash!("items/ghost.jpg"),
                load_reference_hash!("items/ghost_1.jpg"),
            ],
            threshold: 12,
        },
        ItemReference {
            item: Item::GoldenMushroom,
            files: vec![
                load_reference_hash!("items/golden-mushroom.jpg"),
                load_reference_hash!("items/golden-mushroom_1.jpg"),
                load_reference_hash!("items/golden-mushroom_2.jpg"),
                load_reference_hash!("items/golden-mushroom_3.jpg"),
                load_reference_hash!("items/golden-mushroom_4.jpg"),
                load_reference_hash!("items/golden-mushroom_5.jpg"),
            ],
            threshold: 14,
        },
        ItemReference {
            item: Item::GreenShellDouble,
            files: vec![
                load_reference_hash!("items/green-shell-double.jpg"),
                load_reference_hash!("items/green-shell-double_1.jpg"),
                load_reference_hash!("items/green-shell-double_2.jpg"),
                load_reference_hash!("items/green-shell-double_3.jpg"),
            ],
            threshold: 12,
        },
        ItemReference {
            item: Item::GreenShellTriple,
            files: vec![
                load_reference_hash!("items/green-shell-triple.jpg"),
                load_reference_hash!("items/green-shell-triple_1.jpg"),
                load_reference_hash!("items/green-shell-triple_2.jpg"),
                load_reference_hash!("items/green-shell-triple_3.jpg"),
                load_reference_hash!("items/green-shell-triple_4.jpg"),
                load_reference_hash!("items/green-shell-triple_5.jpg"),
            ],
            threshold: 12,
        },
        ItemReference {
            item: Item::GreenShell,
            files: vec![
                load_reference_hash!("items/green-shell.jpg"),
                load_reference_hash!("items/green-shell_1.jpg"),
                load_reference_hash!("items/green-shell_2.jpg"),
                load_reference_hash!("items/green-shell_3.jpg"),
                load_reference_hash!("items/green-shell_4.jpg"),
                load_reference_hash!("items/green-shell_5.jpg"),
            ],
            threshold: 14,
        },
        ItemReference {
            item: Item::Horn,
            files: vec![
                load_reference_hash!("items/horn.jpg"),
                load_reference_hash!("items/horn_1.jpg"),
                load_reference_hash!("items/horn_2.jpg"),
                load_reference_hash!("items/horn_3.jpg"),
                load_reference_hash!("items/horn_4.jpg"),
            ],
            threshold: 12,
        },
        ItemReference {
            item: Item::Lightning,
            files: vec![
                load_reference_hash!("items/lightning.jpg"),
                load_reference_hash!("items/lightning_1.jpg"),
            ],
            threshold: 12,
        },
        ItemReference {
            item: Item::MushroomDouble,
            files: vec![
                load_reference_hash!("items/mushroom-double.jpg"),
                load_reference_hash!("items/mushroom-double_1.jpg"),
                load_reference_hash!("items/mushroom-double_2.jpg"),
                load_reference_hash!("items/mushroom-double_3.jpg"),
            ],
            threshold: 14,
        },
        ItemReference {
            item: Item::MushroomTriple,
            files: vec![
                load_reference_hash!("items/mushroom-triple.jpg"),
                load_reference_hash!("items/mushroom-triple_1.jpg"),
                load_reference_hash!("items/mushroom-triple_2.jpg"),
                load_reference_hash!("items/mushroom-triple_3.jpg"),
                load_reference_hash!("items/mushroom-triple_4.jpg"),
            ],
            threshold: 14,
        },
        ItemReference {
            item: Item::Mushroom,
            files: vec![
                load_reference_hash!("items/mushroom.jpg"),
                load_reference_hash!("items/mushroom_1.jpg"),
                load_reference_hash!("items/mushroom_2.jpg"),
                load_reference_hash!("items/mushroom_3.jpg"),
                load_reference_hash!("items/mushroom_4.jpg"),
            ],
            threshold: 14,
        },
        ItemReference {
            item: Item::PiranhaPlant,
            files: vec![
                load_reference_hash!("items/pirhana-plant.jpg"),
                load_reference_hash!("items/pirhana-plant_1.jpg"),
                load_reference_hash!("items/pirhana-plant_2.jpg"),
            ],
            threshold: 14,
        },
        ItemReference {
            item: Item::RedShellDouble,
            files: vec![load_reference_hash!("items/red-shell-double.jpg"),],
            threshold: 12,
        },
        ItemReference {
            item: Item::RedShellTriple,
            files: vec![
                load_reference_hash!("items/red-shell-triple.jpg"),
                load_reference_hash!("items/red-shell-triple_1.jpg"),
            ],
            threshold: 12,
        },
        ItemReference {
            item: Item::RedShell,
            files: vec![
                load_reference_hash!("items/red-shell.jpg"),
                load_reference_hash!("items/red-shell_1.jpg"),
                load_reference_hash!("items/red-shell_2.jpg"),
                load_reference_hash!("items/red-shell_3.jpg"),
            ],
            threshold: 16,
        },
        ItemReference {
            item: Item::Squid,
            files: vec![load_reference_hash!("items/squid.jpg"),],
            threshold: 12,
        },
        ItemReference {
            item: Item::Star,
            files: vec![
                load_reference_hash!("items/star.jpg"),
                load_reference_hash!("items/star_1.jpg"),
                load_reference_hash!("items/star_2.jpg"),
                load_reference_hash!("items/star_3.jpg"),
                load_reference_hash!("items/star_4.jpg"),
            ],
            threshold: 12,
        },
    ];
}

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

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Race {
    #[serde(serialize_with = "player_vec_serializer", flatten)]
    pub players: Vec<Player>,

    #[serde(skip_serializing)]
    pub starting: bool,
}

impl Display for Race {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = self
            .players
            .iter()
            .fold(Ok(()), |res, player| res.and(writeln!(f, "{}", player)));

        if self.starting {
            res.and(writeln!(f, "(Race Starting!)"))
        } else {
            res
        }
    }
}

const POSITION_CROP: [[u32; 2]; 4] = [[57, 239], [1167, 239], [57, 599], [1167, 599]];
const LAP_FLAG_CROP: [[u32; 2]; 4] = [[114, 317], [1178, 317], [114, 677], [1178, 677]];
const FINISH_CROP: [[u32; 2]; 4] = [[159, 127], [799, 127], [159, 487], [799, 487]];
const ITEM_CROP: [[u32; 2]; 4] = [[99, 62], [1140, 62], [99, 422], [1140, 422]];

#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Racing,

    #[serde(rename = "finish")]
    Finished,
}

impl Status {
    pub fn is_racing(value: &Self) -> bool {
        value == &Status::Racing
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
pub struct Player {
    #[serde(skip_serializing)]
    index: u8,

    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<u8>,

    #[serde(skip_serializing_if = "Status::is_racing")]
    status: Status,

    #[serde(skip_serializing_if = "Option::is_none")]
    item: Option<Item>,
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let item = match self.item {
            Some(item) => format!("- {:?}", item),
            None => "".to_owned(),
        };

        write!(
            f,
            "Player #{}: Position: {:?} ({:?}) {}",
            self.index + 1,
            self.position,
            self.status,
            item,
        )
    }
}

impl Reference for Race {
    fn process(frame: &image::DynamicImage) -> Option<Screen> {
        let mut players: Vec<Player> = (0..4)
            .par_bridge()
            .filter(|p| check_player_exists(&frame, p))
            .map(|p| {
                let position = get_position(&frame, p);

                let status = get_status(&frame, p);

                let item = get_item(&frame, p);

                Player {
                    index: p as u8,
                    position,
                    status,
                    item,
                }
            })
            .collect();

        if players.is_empty() {
            return None;
        }

        players.sort_unstable_by(|a, b| a.index.cmp(&b.index));

        let starting = check_starting(&frame);

        Some(Screen::Race(Race { players, starting }))
    }

    fn compare(frame: &image::DynamicImage) -> bool {
        if !is_splitscreen(frame) {
            return false;
        }

        let width = frame.width();
        let [r, g, b] = average_colors(&frame.crop_imm(width / 2 - 1, 48, 1, 8));
        let average = (r as u32 + g as u32 + b as u32) / 3;

        return average < 5000;
    }
}

fn check_starting(frame: &image::DynamicImage) -> bool {
    let image = frame.crop_imm(573, 292, 39, 37);
    let hash = hasher::hash_image(image);

    GO_REFERENCE.dist(&hash) < 15
}

fn check_player_exists(frame: &image::DynamicImage, index: &usize) -> bool {
    let [x, y] = LAP_FLAG_CROP[*index];
    let image = frame.crop_imm(x, y, 11, 11);

    let hash = hasher::hash_image(image);

    return LAP_FLAG_REFERENCE.dist(&hash) < 20;
}

fn get_position(frame: &image::DynamicImage, index: usize) -> Option<u8> {
    let [x, y] = POSITION_CROP[index];

    let image = frame.crop_imm(x, y, 36, 54).grayscale();
    let res = hasher::hash_image(image);

    REFERENCE_HASHES
        .par_iter()
        .enumerate()
        .map(|(i, hash)| {
            (
                i,
                hash.files
                    .iter()
                    .map(|f| f.dist(&res))
                    .min()
                    .unwrap_or(u32::MAX),
            )
        })
        .filter(|(_, min_dist)| min_dist < &16)
        .min_by(|(_, dist_a), (_, dist_b)| dist_a.cmp(dist_b))
        .map(|(i, _)| (i as u8) + 1)
}

// These are a bit too hard-coded right now, but that's okay
fn check_player_one(frame: &image::DynamicImage) -> bool {
    // check three pixels are pretty close
    let check_slice = frame.crop_imm(112, 40, 10, 2);
    let [average_red, average_green, _] = average_colors(&check_slice);

    average_red > COLOR_THRESHOLD && average_green > COLOR_THRESHOLD
}

fn check_player_two(frame: &image::DynamicImage) -> bool {
    // check three pixels are pretty close
    let check_slice = frame.crop_imm(1154, 40, 10, 2);
    let [_, average_green, average_blue] = average_colors(&check_slice);

    average_green > COLOR_THRESHOLD && average_blue > COLOR_THRESHOLD
}

fn check_player_three(frame: &image::DynamicImage) -> bool {
    // check three pixels are pretty close
    let check_slice = frame.crop_imm(112, 400, 10, 2);
    let [average_red, _, _] = average_colors(&check_slice);

    average_red > COLOR_THRESHOLD
}
fn check_player_four(frame: &image::DynamicImage) -> bool {
    // check three pixels are pretty close
    let check_slice = frame.crop_imm(1154, 400, 10, 2);
    let [_, average_green, _] = average_colors(&check_slice);

    average_green > COLOR_THRESHOLD
}

fn check_player(frame: &image::DynamicImage, index: usize) -> bool {
    match index {
        0 => check_player_one(frame),
        1 => check_player_two(frame),
        2 => check_player_three(frame),
        3 => check_player_four(frame),
        _ => false,
    }
}

fn get_item(frame: &image::DynamicImage, index: usize) -> Option<Item> {
    if !check_player(frame, index) {
        return None;
    }

    let [x, y] = ITEM_CROP[index];
    let image = frame.crop_imm(x, y, 41, 41);

    let res = hasher::hash_image(image);

    ITEM_HASHES
        .par_iter()
        .map(|hash| {
            (
                hash,
                hash.files
                    .iter()
                    .map(|f| f.dist(&res))
                    .min()
                    .unwrap_or(u32::MAX),
            )
        })
        .filter(|(h, dist)| dist <= &(h.threshold as u32))
        // there is maybe something we can do here with threshold - dist
        .min_by(|(_, dist_a), (_, dist_b)| dist_a.cmp(dist_b))
        .map(|(hash, _)| hash)
        .map(|hash| hash.item)
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

#[cfg(test)]
mod tests {
    use crate::reference::Reference;
    use pretty_assertions::assert_eq;

    use super::{Item, Player, Race, Status};

    macro_rules! player {
        ($index:expr) => {
            Player {
                index: $index,
                position: None,
                status: Status::Racing,
                item: None,
            }
        };
        ($index:expr, $pos:expr) => {
            Player {
                index: $index,
                position: Some($pos),
                status: Status::Racing,
                item: None,
            }
        };
        ($index:expr, $pos:expr, $item:ident) => {
            Player {
                index: $index,
                position: Some($pos),
                status: Status::Racing,
                item: Some(Item::$item),
            }
        };
        ($index:expr, $pos:expr, $item:ident, $status:ident) => {
            Player {
                index: $index,
                position: Some($pos),
                status: Status::$status,
                item: Some(Item::$item),
            }
        };
    }

    macro_rules! test_race {
        ($name:ident, $($players:expr,)*) => {
            #[test]
            fn $name() {
                let image_data = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/spec-data/screens/race/", stringify!($name), ".jpg"));
                let frame = image::load_from_memory(image_data).expect("failed to open image");
                let result = Race::process(&frame);

                assert_eq!(
                    Some(super::Screen::Race(Race {
                        players: vec![
                            $($players),+
                        ],
                        starting: false,
                    })),
                    result,
                );
            }
        };
    }

    test_race!(
        test_1,
        player!(0, 7, PiranhaPlant),
        player!(1, 9, GreenShell),
        player!(2, 4, GreenShell),
        player!(3, 2, BananaDouble),
    );

    test_race!(
        test_2,
        player!(0, 5),
        player!(1, 8, GreenShell),
        player!(2, 3),
        player!(3, 1),
    );

    test_race!(
        test_3,
        player!(0),
        player!(1, 12),
        player!(2, 11),
        player!(3),
    );

    test_race!(
        test_4,
        Player {
            index: 0,
            status: Status::Finished,
            item: None,
            position: Some(6)
        },
        player!(1, 2),
        Player {
            index: 2,
            status: Status::Finished,
            item: None,
            position: Some(4)
        },
        player!(3, 9, Mushroom),
    );

    test_race!(
        test_5,
        player!(0, 8, GreenShell),
        player!(1, 2, RedShell),
        player!(2, 7, RedShell),
        Player {
            index: 3,
            status: Status::Racing,
            item: Some(Item::GreenShell),
            position: None // mid transition
        },
    );

    test_race!(
        test_6,
        player!(0, 5, Mushroom),
        player!(1, 8, Mushroom),
        player!(2, 2, GreenShell),
        player!(3, 6, Banana),
    );

    test_race!(
        test_7,
        player!(0, 11),
        player!(1, 12, GoldenMushroom),
        player!(2, 7),
    );

    test_race!(
        test_8,
        player!(0, 12, Bullet),
        player!(1, 10, RedShell),
        player!(2, 6),
    );
}
