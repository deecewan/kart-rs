use crate::color::average_colors;
use crate::hasher;
use crate::load_reference_hash;
use crate::reference::Reference;
use crate::screens::Screen;
use crate::util::print_dynamic_image;
use image::GenericImage;
use image::GenericImageView;
use lazy_static::lazy_static;
use rayon::prelude::*;

pub struct VariantGroup<'a> {
    pub variant: image_hasher::ImageHash,
    pub tracks: Vec<IntroReference<'a>>,
}

pub struct IntroReference<'a> {
    pub name: &'a str,
    pub reference: image_hasher::ImageHash,
}

lazy_static! {
    static ref REFERENCE_HASH: image_hasher::ImageHash =
        load_reference_hash!("intro/intro_reference.jpg");
    pub static ref VARIANT_GROUPS: Vec<VariantGroup<'static>> = vec![
        VariantGroup {
            variant: load_reference_hash!("intro/variants/3ds.png"),
            tracks: vec![
                IntroReference {
                    name: "Alpine Pass (3DS)",
                    reference: load_reference_hash!("intro/tracks/alpine_pass_3ds.jpg"),
                },
                IntroReference {
                    name: "DK Jungle (3DS)",
                    reference: load_reference_hash!("intro/tracks/dk_jungle_3ds.jpg"),
                },
                IntroReference {
                    name: "Koopa City (3DS)",
                    reference: load_reference_hash!("intro/tracks/koopa_city_3ds.jpg"),
                },
                IntroReference {
                    name: "Melody Motorway (3DS)",
                    reference: load_reference_hash!("intro/tracks/melody_motorway_3ds.jpg"),
                },
                IntroReference {
                    name: "Music Park (3DS)",
                    reference: load_reference_hash!("intro/tracks/music_park_3ds.jpg"),
                },
                IntroReference {
                    name: "Neo Bowser City (3DS)",
                    reference: load_reference_hash!("intro/tracks/neo_bowser_city_3ds.jpg"),
                },
                IntroReference {
                    name: "Piranha Plant Slide (3DS)",
                    reference: load_reference_hash!("intro/tracks/piranha_plant_slide_3ds.jpg"),
                },
                IntroReference {
                    name: "Rainbow Road (3DS)",
                    reference: load_reference_hash!("intro/tracks/rainbow_road_3ds.jpg"),
                },
                IntroReference {
                    name: "Rock Rock Mountain (3DS)",
                    reference: load_reference_hash!("intro/tracks/rock_rock_mountain_3ds.jpg"),
                },
                IntroReference {
                    name: "Toad Circuit (3DS)",
                    reference: load_reference_hash!("intro/tracks/toad_circuit_3ds.jpg"),
                },
            ],
        },
        VariantGroup {
            variant: load_reference_hash!("intro/variants/ds.png"),
            tracks: vec![
                IntroReference {
                    name: "Cheep Cheep Beach (DS)",
                    reference: load_reference_hash!("intro/tracks/cheep_cheep_beach_ds.jpg"),
                },
                IntroReference {
                    name: "Peach Gardens (DS)",
                    reference: load_reference_hash!("intro/tracks/peach_gardens_ds.jpg"),
                },
                IntroReference {
                    name: "Shroom Ridge (DS)",
                    reference: load_reference_hash!("intro/tracks/shroom_ridge_ds.jpg"),
                },
                IntroReference {
                    name: "Tick-Tock Clock (DS)",
                    reference: load_reference_hash!("intro/tracks/tick-tock_clock_ds.jpg"),
                },
                IntroReference {
                    name: "Waluigi Pinball (DS)",
                    reference: load_reference_hash!("intro/tracks/waluigi_pinball_ds.jpg"),
                },
                IntroReference {
                    name: "Wario Stadium (DS)",
                    reference: load_reference_hash!("intro/tracks/wario_stadium_ds.jpg"),
                },
            ],
        },
        VariantGroup {
            variant: load_reference_hash!("intro/variants/gba.png"),
            tracks: vec![
                IntroReference {
                    name: "Boo Lake (GBA)",
                    reference: load_reference_hash!("intro/tracks/boo_lake_gba.jpg"),
                },
                IntroReference {
                    name: "Cheese Land (GBA)",
                    reference: load_reference_hash!("intro/tracks/cheese_land_gba.jpg"),
                },
                IntroReference {
                    name: "Mario Circuit (GBA)",
                    reference: load_reference_hash!("intro/tracks/mario_circuit_gba.jpg"),
                },
                IntroReference {
                    name: "Ribbon Road (GBA)",
                    reference: load_reference_hash!("intro/tracks/ribbon_road_gba.jpg"),
                },
                IntroReference {
                    name: "Sky Garden (GBA)",
                    reference: load_reference_hash!("intro/tracks/sky_garden_gba.jpg"),
                },
                IntroReference {
                    name: "Snow Land (GBA)",
                    reference: load_reference_hash!("intro/tracks/snow_land_gba.jpg"),
                },
            ],
        },
        VariantGroup {
            variant: load_reference_hash!("intro/variants/gcn.png"),
            tracks: vec![
                IntroReference {
                    name: "Baby Park (GameCube)",
                    reference: load_reference_hash!("intro/tracks/baby_park_gcn.jpg"),
                },
                IntroReference {
                    name: "Dry Dry Desert (GameCube)",
                    reference: load_reference_hash!("intro/tracks/dry_dry_desert_gcn.jpg"),
                },
                IntroReference {
                    name: "Sherbet Land (GameCube)",
                    reference: load_reference_hash!("intro/tracks/sherbet_land_gcn.jpg"),
                },
                IntroReference {
                    name: "Yoshi Circuit (GameCube)",
                    reference: load_reference_hash!("intro/tracks/yoshi_circuit_gcn.jpg"),
                },
            ],
        },
        VariantGroup {
            variant: load_reference_hash!("intro/variants/n64.png"),
            tracks: vec![
                IntroReference {
                    name: "Choco Mountain (N64)",
                    reference: load_reference_hash!("intro/tracks/choco_mountain_n64.jpg"),
                },
                IntroReference {
                    name: "Kalimari Desert (N64)",
                    reference: load_reference_hash!("intro/tracks/kalimari_desert_n64.jpg"),
                },
                IntroReference {
                    name: "Rainbow Road (N64)",
                    reference: load_reference_hash!("intro/tracks/rainbow_road_n64.jpg"),
                },
                IntroReference {
                    name: "Royal Raceway (N64)",
                    reference: load_reference_hash!("intro/tracks/royal_raceway_n64.jpg"),
                },
                IntroReference {
                    name: "Toad's Turnpike (N64)",
                    reference: load_reference_hash!("intro/tracks/toads_turnpike_n64.jpg"),
                },
                IntroReference {
                    name: "Yoshi Valley (N64)",
                    reference: load_reference_hash!("intro/tracks/yoshi_valley_n64.jpg"),
                },
            ],
        },
        VariantGroup {
            variant: load_reference_hash!("intro/variants/none.png"),
            tracks: vec![
                IntroReference {
                    name: "Animal Crossing",
                    reference: load_reference_hash!("intro/tracks/animal_crossing.jpg"),
                },
                IntroReference {
                    name: "Big Blue",
                    reference: load_reference_hash!("intro/tracks/big_blue.jpg"),
                },
                IntroReference {
                    name: "Bone Dry Dunes",
                    reference: load_reference_hash!("intro/tracks/bone-dry_dunes.jpg"),
                },
                IntroReference {
                    name: "Bowser's Castle",
                    reference: load_reference_hash!("intro/tracks/bowsers_castle.jpg"),
                },
                IntroReference {
                    name: "Cloudtop Cruise",
                    reference: load_reference_hash!("intro/tracks/cloudtop_cruise.jpg"),
                },
                IntroReference {
                    name: "Dolphin Shoals",
                    reference: load_reference_hash!("intro/tracks/dolphin_shoals.jpg"),
                },
                IntroReference {
                    name: "Dragon Driftway",
                    reference: load_reference_hash!("intro/tracks/dragon_driftway.jpg"),
                },
                IntroReference {
                    name: "Electrodrome",
                    reference: load_reference_hash!("intro/tracks/electrodrome.jpg"),
                },
                IntroReference {
                    name: "Excitebike Arena",
                    reference: load_reference_hash!("intro/tracks/excitebike_arena.jpg"),
                },
                IntroReference {
                    name: "Hyrule Circuit",
                    reference: load_reference_hash!("intro/tracks/hyrule_circuit.jpg"),
                },
                IntroReference {
                    name: "Ice Ice Outpost",
                    reference: load_reference_hash!("intro/tracks/ice_ice_outpost.jpg"),
                },
                IntroReference {
                    name: "Mario Circuit",
                    reference: load_reference_hash!("intro/tracks/mario_circuit.jpg"),
                },
                IntroReference {
                    name: "Mario Kart Stadium",
                    reference: load_reference_hash!("intro/tracks/mario_kart_stadium.jpg"),
                },
                IntroReference {
                    name: "Merry Mountain",
                    reference: load_reference_hash!("intro/tracks/merry_mountain.jpg"),
                },
                IntroReference {
                    name: "Mount Wario",
                    reference: load_reference_hash!("intro/tracks/mount_wario.jpg"),
                },
                IntroReference {
                    name: "Mute City",
                    reference: load_reference_hash!("intro/tracks/mute_city.jpg"),
                },
                IntroReference {
                    name: "Ninja Hideaway",
                    reference: load_reference_hash!("intro/tracks/ninja_hideaway.jpg"),
                },
                IntroReference {
                    name: "Rainbow Road",
                    reference: load_reference_hash!("intro/tracks/rainbow_road.jpg"),
                },
                IntroReference {
                    name: "Shy Guy Falls",
                    reference: load_reference_hash!("intro/tracks/shy_guy_falls.jpg"),
                },
                IntroReference {
                    name: "Sky-High Sundae",
                    reference: load_reference_hash!("intro/tracks/skyhigh_sundae.jpg"),
                },
                IntroReference {
                    name: "Sunshine Airport",
                    reference: load_reference_hash!("intro/tracks/sunshine_airport.jpg"),
                },
                IntroReference {
                    name: "Super Bell Subway",
                    reference: load_reference_hash!("intro/tracks/super_bell_subway.jpg"),
                },
                IntroReference {
                    name: "Sweet Sweet Canyon",
                    reference: load_reference_hash!("intro/tracks/sweet_sweet_canyon.jpg"),
                },
                IntroReference {
                    name: "Thwomp Ruins",
                    reference: load_reference_hash!("intro/tracks/thwomp_ruins.jpg"),
                },
                IntroReference {
                    name: "Toad Harbor",
                    reference: load_reference_hash!("intro/tracks/toad_harbor.jpg"),
                },
                IntroReference {
                    name: "Twisted Mansion",
                    reference: load_reference_hash!("intro/tracks/twisted_mansion.jpg"),
                },
                IntroReference {
                    name: "Water Park",
                    reference: load_reference_hash!("intro/tracks/water_park.jpg"),
                },
                IntroReference {
                    name: "Wild Woods",
                    reference: load_reference_hash!("intro/tracks/wild_woods.jpg"),
                },
            ],
        },
        VariantGroup {
            variant: load_reference_hash!("intro/variants/snes.png"),
            tracks: vec![
                IntroReference {
                    name: "Donut Plains 3 (SNES)",
                    reference: load_reference_hash!("intro/tracks/donut_plains_3_snes.jpg"),
                },
                IntroReference {
                    name: "Mario Circuit 3 (SNES)",
                    reference: load_reference_hash!("intro/tracks/mario_circuit_3_snes.jpg"),
                },
                IntroReference {
                    name: "Rainbow Road (SNES)",
                    reference: load_reference_hash!("intro/tracks/rainbow_road_snes.jpg"),
                },
            ],
        },
        VariantGroup {
            variant: load_reference_hash!("intro/variants/tour.png"),
            tracks: vec![
                IntroReference {
                    name: "Berlin Byways (Tour)",
                    reference: load_reference_hash!("intro/tracks/berlin_byways_tour.jpg"),
                },
                IntroReference {
                    name: "London Loop (Tour)",
                    reference: load_reference_hash!("intro/tracks/london_loop_tour.jpg"),
                },
                IntroReference {
                    name: "New York Minute (Tour)",
                    reference: load_reference_hash!("intro/tracks/new_york_minute_tour.jpg"),
                },
                IntroReference {
                    name: "Paris Promenade (Tour)",
                    reference: load_reference_hash!("intro/tracks/paris_promenade_tour.jpg"),
                },
                IntroReference {
                    name: "Sydney Sprint (Tour)",
                    reference: load_reference_hash!("intro/tracks/sydney_sprint_tour.jpg"),
                },
                IntroReference {
                    name: "Tokyo Blur (Tour)",
                    reference: load_reference_hash!("intro/tracks/tokyo_blur_tour.jpg"),
                },
            ],
        },
        VariantGroup {
            variant: load_reference_hash!("intro/variants/wii.png"),
            tracks: vec![
                IntroReference {
                    name: "Coconut Mall (Wii)",
                    reference: load_reference_hash!("intro/tracks/coconut_mall_wii.jpg"),
                },
                IntroReference {
                    name: "Grumble Volcano (Wii)",
                    reference: load_reference_hash!("intro/tracks/grumble_volcano_wii.jpg"),
                },
                IntroReference {
                    name: "Maple Treeway (Wii)",
                    reference: load_reference_hash!("intro/tracks/maple_treeway_wii.jpg"),
                },
                IntroReference {
                    name: "Moo Moo Meadows (Wii)",
                    reference: load_reference_hash!("intro/tracks/moo_moo_meadows_wii.jpg"),
                },
                IntroReference {
                    name: "Mushroom Gorge (Wii)",
                    reference: load_reference_hash!("intro/tracks/mushroom_gorge_wii.jpg"),
                },
                IntroReference {
                    name: "Wario’s Gold Mine (Wii)",
                    reference: load_reference_hash!("intro/tracks/warios_gold_mine_wii.jpg"),
                },
            ],
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

        if delta > 5 {
            return false;
        }

        return check_speed_slice(frame);
    }

    fn process(frame: &image::DynamicImage) -> Option<Screen> {
        let variant = get_variant_image(&frame);
        let track = get_track_image(&frame);

        let closest_track = find_closest_track(variant, track);

        let course = match closest_track {
            None => {
                let now = std::time::SystemTime::now();
                let timestamp = now
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("time went backwards")
                    .as_millis();
                let filename = format!("unknown-intros/{}.jpg", timestamp);
                frame.save(filename).expect("Failed to save unknown course");
                "Unknown Course"
            }
            Some(course) => course,
        };

        Some(Screen::Intro(Intro { course }))
    }
}

fn check_speed_slice(frame: &image::DynamicImage) -> bool {
    // make sure the speed indicator shows on the right side, otherwise
    // there is no text, so the track will be unknown
    let speed_slice = frame.crop_imm(1130, 600, 10, 2);

    let [r, g, b] = average_colors(&speed_slice);

    return r > 60_000 && g > 60_000 && b > 60_000;
}

pub fn get_variant_image(image: &image::DynamicImage) -> image::DynamicImage {
    let mut variant = image.crop_imm(258, 638, 80, 18);
    set_black_pixels(&mut variant);

    return variant;
}

pub fn get_track_image(image: &image::DynamicImage) -> image::DynamicImage {
    let mut track = image.crop_imm(338, 620, 350, 36);
    set_black_pixels(&mut track);

    return track;
}

fn find_closest_track(
    variant: image::DynamicImage,
    track: image::DynamicImage,
) -> Option<&'static str> {
    let variant_hash = hasher::hash_image(variant);

    VARIANT_GROUPS
        .iter()
        // from some testing, non-matching variants are well over 30, so we're
        // unlikely to have multiple variants under 10 - no need to filter + min
        .find(|g| {
            let dist = g.variant.dist(&variant_hash);

            return dist < 10;
        })
        .map(|group| {
            let track_hash = hasher::hash_image(track);

            group
                .tracks
                .par_iter()
                .map(|t| {
                    let dist = t.reference.dist(&track_hash);

                    (t.name, dist)
                })
                .filter(|(_, dist)| dist < &10)
                .min_by(|(_, a), (_, b)| a.cmp(b))
                .map(|(name, _)| name)
        })
        .flatten()
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

#[cfg(test)]
mod tests {
    use super::Intro;
    use crate::reference::Reference;

    #[test]
    fn ignores_before_title() {
        let path = "spec_data/lib/screens/intro/no_track_name.jpg";
        let frame = image::open(path).expect("failed to open image");
        let result = Intro::compare(&frame);

        assert!(!result)
    }

    #[test]
    fn ignores_partial_title() {
        let path = "spec_data/lib/screens/intro/partial_track_name.jpg";
        let frame = image::open(path).expect("failed to open image");
        let result = Intro::compare(&frame);

        assert!(!result)
    }

    #[test]
    fn approves_full_title() {
        let path = "spec_data/lib/screens/intro/full_track_name.jpg";
        let frame = image::open(path).expect("failed to open image");
        let result = Intro::compare(&frame);

        assert!(result)
    }
}
