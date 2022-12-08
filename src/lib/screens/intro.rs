use crate::hasher;
use crate::load_reference_hash;
use crate::reference::Reference;
use crate::screens::Screen;
use image::GenericImage;
use image::GenericImageView;
use lazy_static::lazy_static;
use rayon::prelude::*;

struct IntroReference<'a> {
    name: &'a str,
    files: Vec<image_hasher::ImageHash>,
}

lazy_static! {
    static ref REFERENCE_HASH: image_hasher::ImageHash =
        load_reference_hash!("intro/intro_reference.jpg");
    static ref INTRO_HASHES: Vec<IntroReference<'static>> = vec![
        IntroReference {
            name: "Sunshine Airport",
            files: vec![load_reference_hash!("intro/sunshine_airport.jpg")],
        },
        IntroReference {
            name: "Dolphin Shoals",
            files: vec![load_reference_hash!("intro/dolphin_shoals.jpg")],
        },
        IntroReference {
            name: "Electrodrome",
            files: vec![load_reference_hash!("intro/electrodrome.jpg")],
        },
        IntroReference {
            name: "Mount Wario",
            files: vec![load_reference_hash!("intro/mount_wario.jpg")],
        },
        IntroReference {
            name: "Moo Moo Meadows (Wii)",
            files: vec![load_reference_hash!("intro/moo_moo_meadows.jpg")],
        },
        IntroReference {
            name: "Mario Circuit (GBA)",
            files: vec![load_reference_hash!("intro/mario_circuit_gba.jpg")],
        },
        IntroReference {
            name: "Cheep Cheep Beach (DS)",
            files: vec![load_reference_hash!("intro/cheep_cheep_beach.jpg")],
        },
        IntroReference {
            name: "Toad's Turnpike (N64)",
            files: vec![load_reference_hash!("intro/toads_turnpike.jpg")],
        },
        IntroReference {
            name: "Mario Circuit",
            files: vec![load_reference_hash!("intro/mario_circuit.jpg")],
        },
        IntroReference {
            name: "Toad Harbor",
            files: vec![load_reference_hash!("intro/toad_harbor.jpg")],
        },
        IntroReference {
            name: "Twisted Mansion",
            files: vec![load_reference_hash!("intro/twisted_mansion.jpg")],
        },
        IntroReference {
            name: "Shy Guy Falls",
            files: vec![load_reference_hash!("intro/shy_guy_falls.jpg")],
        },
        IntroReference {
            name: "Cloudtop Cruise",
            files: vec![load_reference_hash!("intro/cloudtop_cruise.jpg")],
        },
        IntroReference {
            name: "Bone Dry Dunes",
            files: vec![load_reference_hash!("intro/bone_dry_dunes.jpg")],
        },
        IntroReference {
            name: "Bowser's Castle",
            files: vec![load_reference_hash!("intro/bowsers_castle.jpg")],
        },
        IntroReference {
            name: "Rainbow Road",
            files: vec![load_reference_hash!("intro/rainbow_road.jpg")],
        },
        IntroReference {
            name: "Mario Kart Stadium",
            files: vec![load_reference_hash!("intro/mario_kart_stadium.jpg")],
        },
        IntroReference {
            name: "Water Park",
            files: vec![load_reference_hash!("intro/water_park.jpg")],
        },
        IntroReference {
            name: "Sweet Sweet Canyon",
            files: vec![load_reference_hash!("intro/sweet_sweet_canyon.jpg")],
        },
        IntroReference {
            name: "Thwomp Ruins",
            files: vec![load_reference_hash!("intro/thwomp_ruins.jpg")],
        },
        IntroReference {
            name: "Dry Dry Desert (GameCube)",
            files: vec![load_reference_hash!("intro/dry_dry_desert.jpg")],
        },
        IntroReference {
            name: "Donut Plains 3 (SNES)",
            files: vec![load_reference_hash!("intro/donut_plains_3.jpg")],
        },
        IntroReference {
            name: "Royal Raceway (N64)",
            files: vec![load_reference_hash!("intro/royal_raceway.jpg")],
        },
        IntroReference {
            name: "DK Jungle (3DS)",
            files: vec![load_reference_hash!("intro/dk_jungle.jpg")],
        },
        IntroReference {
            name: "Wario Stadium (DS)",
            files: vec![load_reference_hash!("intro/wario_stadium.jpg")],
        },
        IntroReference {
            name: "Sherbet Land (GameCube)",
            files: vec![load_reference_hash!("intro/sherbet_land.jpg")],
        },
        IntroReference {
            name: "Melody Motorway (3DS)",
            files: vec![load_reference_hash!("intro/melody_motorway.jpg")],
        },
        IntroReference {
            name: "Yoshi Valley (N64)",
            files: vec![load_reference_hash!("intro/yoshi_valley.jpg")],
        },
        IntroReference {
            name: "Tick-Tock Clock (DS)",
            files: vec![load_reference_hash!("intro/tick_tock_clock.jpg")],
        },
        IntroReference {
            name: "Piranha Plant Slide (3DS)",
            files: vec![load_reference_hash!("intro/piranha_plant_slide.jpg")],
        },
        IntroReference {
            name: "Grumble Volcano (Wii)",
            files: vec![load_reference_hash!("intro/grumble_volcano.jpg")],
        },
        IntroReference {
            name: "Rainbow Road (N64)",
            files: vec![
                load_reference_hash!("intro/rainbow_road_n642.jpg"),
                load_reference_hash!("intro/rainbow_road_n64.jpg")
            ],
        },
        IntroReference {
            name: "Yoshi Circuit (GameCube)",
            files: vec![load_reference_hash!("intro/yoshi_circuit.jpg")],
        },
        IntroReference {
            name: "Excitebike Arena",
            files: vec![load_reference_hash!("intro/excitebike_arena.jpg")],
        },
        IntroReference {
            name: "Dragon Driftway",
            files: vec![load_reference_hash!("intro/dragon_driftway.jpg")],
        },
        IntroReference {
            name: "Mute City",
            files: vec![load_reference_hash!("intro/mute_city.jpg")],
        },
        IntroReference {
            name: "Wario’s Gold Mine (Wii)",
            files: vec![load_reference_hash!("intro/warios_gold_mine.jpg")],
        },
        IntroReference {
            name: "Rainbow Road (SNES)",
            files: vec![load_reference_hash!("intro/rainbow_road_snes.jpg")],
        },
        IntroReference {
            name: "Ice Ice Outpost",
            files: vec![load_reference_hash!("intro/ice_ice_outpost.jpg")],
        },
        IntroReference {
            name: "Hyrule Circuit",
            files: vec![load_reference_hash!("intro/hyrule_circuit.jpg")],
        },
        IntroReference {
            name: "Baby Park (GameCube)",
            files: vec![load_reference_hash!("intro/baby_park.jpg")],
        },
        IntroReference {
            name: "Cheese Land (GBA)",
            files: vec![load_reference_hash!("intro/cheese_land.jpg")],
        },
        IntroReference {
            name: "Wild Woods",
            files: vec![load_reference_hash!("intro/wild_woods.jpg")],
        },
        IntroReference {
            name: "Animal Crossing",
            files: vec![load_reference_hash!("intro/animal_crossing.jpg")],
        },
        IntroReference {
            name: "Koopa City (3DS)",
            files: vec![load_reference_hash!("intro/koopa_city.jpg")],
        },
        IntroReference {
            name: "Ribbon Road (GBA)",
            files: vec![load_reference_hash!("intro/ribbon_road.jpg")],
        },
        IntroReference {
            name: "Super Bell Subway",
            files: vec![load_reference_hash!("intro/super_bell_subway.jpg")],
        },
        IntroReference {
            name: "Big Blue",
            files: vec![load_reference_hash!("intro/big_blue.jpg")],
        },
        IntroReference {
            name: "Paris Promenade (Tour)",
            files: vec![load_reference_hash!("intro/paris_promenade_tour.jpg")],
        },
        IntroReference {
            name: "Toad Circuit (3DS)",
            files: vec![load_reference_hash!("intro/toad_circuit_3ds.jpg")],
        },
        IntroReference {
            name: "Choco Mountain (N64)",
            files: vec![load_reference_hash!("intro/choco_mountain_n64.jpg")],
        },
        IntroReference {
            name: "Coconut Mall (Wii)",
            files: vec![load_reference_hash!("intro/coconut_mall_wii.jpg")],
        },
        IntroReference {
            name: "Tokyo Blur (Tour)",
            files: vec![load_reference_hash!("intro/tokyo_blur_tour.jpg")],
        },
        IntroReference {
            name: "Shroom Ridge (DS)",
            files: vec![load_reference_hash!("intro/shroom_ridge_ds.jpg")],
        },
        IntroReference {
            name: "Sky Garden (GBA)",
            files: vec![load_reference_hash!("intro/sky_garden_gba.jpg")],
        },
        IntroReference {
            name: "Ninja Hideaway",
            files: vec![load_reference_hash!("intro/ninja_hideaway.jpg")],
        },
        IntroReference {
            name: "New York Minute (Tour)",
            files: vec![load_reference_hash!("intro/new_york_minute_tour.jpg")],
        },
        IntroReference {
            name: "Mario Circuit 3 (SNES)",
            files: vec![load_reference_hash!("intro/mario_circuit_3_snes.jpg")],
        },
        IntroReference {
            name: "Kalimari Desert (N64)",
            files: vec![load_reference_hash!("intro/kalimari_desert_n64.jpg")],
        },
        IntroReference {
            name: "Waluigi Pinball (DS)",
            files: vec![load_reference_hash!("intro/waluigi_pinball_ds.jpg")],
        },
        IntroReference {
            name: "Sydney Sprint (Tour)",
            files: vec![load_reference_hash!("intro/sydney_sprint_tour.jpg")],
        },
        IntroReference {
            name: "Snow Land (GBA)",
            files: vec![load_reference_hash!("intro/snow_land_gba.jpg")],
        },
        IntroReference {
            name: "Mushroom Gorge (Wii)",
            files: vec![load_reference_hash!("intro/mushroom_gorge_wii.jpg")],
        },
        IntroReference {
            name: "Sky-High Sundae",
            files: vec![load_reference_hash!("intro/sky_high_sundae.jpg")],
        },
    ];
}

#[derive(Debug, PartialEq)]
pub struct Intro {
    course: &'static str,
}

impl Reference for Intro {
    fn compare(frame: &image::DynamicImage) -> bool {
        let crop = frame.crop_imm(111, 589, 44, 37);

        let check_hash = hasher::hash_image(crop);
        let delta = REFERENCE_HASH.dist(&check_hash);

        return delta <= 10;
    }

    fn process(frame: &image::DynamicImage) -> Option<Screen> {
        let text = get_comparison_image(&frame);
        let closest_track = find_closest_track(text).unwrap_or("Unknown Course");
        Some(Screen::Intro(Intro {
            course: closest_track,
        }))
    }
}

pub fn get_comparison_image(image: &image::DynamicImage) -> image::DynamicImage {
    let mut text = image.crop_imm(258, 620, 350, 36);
    set_black_pixels(&mut text);

    return text;
}

fn find_closest_track(image: image::DynamicImage) -> Option<&'static str> {
    let res = hasher::hash_image(image);
    INTRO_HASHES
        .par_iter()
        .filter_map(|reference| {
            let dist = reference
                .files
                .iter()
                .map(|f| {
                    let dist = f.dist(&res);
                    if dist < 10 {
                        println!("{}: dist {dist}", reference.name);
                    }

                    dist
                })
                .min();

            if dist < Some(10) {
                Some((reference.name, dist))
            } else {
                None
            }
        })
        .min_by(|a, b| a.1.cmp(&b.1))
        .map(|(name, _)| name)
}

fn set_black_pixels(image: &mut image::DynamicImage) {
    for y in 0..image.height() {
        for x in 0..image.width() {
            let px = image.get_pixel(x, y);
            let [r, g, b, _] = px.0;
            let avg = (r as u16 + g as u16 + b as u16) / 3;

            if avg > 220 {
                image.put_pixel(x, y, image::Rgba::from([255, 255, 255, 1]));
            } else {
                image.put_pixel(x, y, image::Rgba::from([0, 0, 0, 1]));
            }
        }
    }
}
