mod color;
mod hasher;
mod reference;
mod screens;
mod util;

use image;
use reference::Reference;
use screens::*;

pub fn analyze(frame: &image::DynamicImage) -> Option<Screen> {
    let resized = frame.resize(1280, 720, image::imageops::Nearest);
    let res = if race::Race::compare(&resized) {
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
    };

    res
}
