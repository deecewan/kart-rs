use super::reference::Reference;
use super::screens::*;

pub fn process(frame: image::DynamicImage) -> Option<Screen> {
    let resized = frame.resize(1280, 720, image::imageops::Nearest);
    if race::Race::compare(&resized) {
        race::Race::process(&resized)
    } else if main_menu::MainMenu::compare(&resized) {
        main_menu::MainMenu::process(&resized)
    } else if race_result::RaceResult::compare(&resized) {
        race_result::RaceResult::process(&resized)
    } else if select_character::SelectCharacter::compare(&resized) {
        select_character::SelectCharacter::process(&resized)
    } else if loading::Loading::compare(&resized) {
        loading::Loading::process(&resized)
    } else if intro::Intro::compare(&resized) {
        intro::Intro::process(&resized)
    } else if match_result::MatchResult::compare(&resized) {
        match_result::MatchResult::process(&resized)
    } else {
        Some(Screen::Unknown)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rayon::prelude::*;

    struct TestStruct<'a> {
        files: Vec<&'a str>,
        reference: Screen,
    }

    #[test]
    fn it_works() {
        let specs = vec![
            TestStruct {
                reference: Screen::MainMenu(main_menu::MainMenu {  }),
                files: vec![
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062120144000-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062119543400-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062619344400-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062119544000-16851BE00BC6068871FE49D98876D6C5.jpg",
                ],
            },
            TestStruct {
                reference: Screen::Race(race::Race { players: vec![] }),
                files: vec![
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062120103000-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062120081300-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062120102300-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062619480400-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062120001500-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062619523000-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062120123400-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062619475100-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062619411900-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062119574800-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062120135500-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062619400100-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062619483701-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062120074500-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062120123700-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062120120200-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062120043300-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062619403400-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062120102000-16851BE00BC6068871FE49D98876D6C5.jpg",
                    // Start race - GO
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062619420500-16851BE00BC6068871FE49D98876D6C5.jpg",
                    // Start race - GO
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062619461400-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062619420500-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062619522600-16851BE00BC6068871FE49D98876D6C5.jpg",
                    // multiple `finish` states
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062619523100-16851BE00BC6068871FE49D98876D6C5.jpg",
                    // this one has an incorrect position detection - p1 detects as 6, is actually 8
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062619484700-16851BE00BC6068871FE49D98876D6C5.jpg"
                ],
            },
            TestStruct {
                reference: Screen::RaceResult(race_result::RaceResult {  }),
                files: vec![
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062120061200-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062120140500-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062120014400-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062619413900-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062120014700-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062120061400-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062619453500-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062619485200-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062120095800-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062619413000-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062619524500-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062120095900-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062120060900-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062619485000-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062120140200-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062120100100-16851BE00BC6068871FE49D98876D6C5.jpg",
                    // THIS one is so cooked? it's like a mid-frame capture
                    // "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062619453800-16851BE00BC6068871FE49D98876D6C5.jpg",
                ],
            },
            TestStruct {
                reference: Screen::MatchResult(match_result::MatchResult {  }),
                files: vec![
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062619530300-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062619530000-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062120142700-16851BE00BC6068871FE49D98876D6C5.jpg",
                ],
            },
            TestStruct {
                reference: Screen::SelectCharacter(select_character::SelectCharacter {  }),
                files: vec![
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062119555900-16851BE00BC6068871FE49D98876D6C5.jpg",
                ],
            },
            TestStruct {
                reference: Screen::Loading(loading::Loading {  }),
                files: vec![
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062120020000-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062119573200-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062619372200-16851BE00BC6068871FE49D98876D6C5.jpg",
                ],
            },
            TestStruct {
                reference: Screen::Intro(intro::Intro {  }),
                files: vec![
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062119574100-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062120020800-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062619373100-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062619491100-16851BE00BC6068871FE49D98876D6C5.jpg",
                    "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062120101700-16851BE00BC6068871FE49D98876D6C5.jpg",
                ],
            }
        ];
        // this is a "race start - GO" screenshot
        // "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062619461400-16851BE00BC6068871FE49D98876D6C5.jpg",

        specs.par_iter().for_each(|spec| {
            spec.files.par_iter().for_each(|file| {
                assert_eq!(
                    process(image::open(file).expect("couldn't load file")).unwrap(),
                    spec.reference,
                );
            });
        });
    }
}
