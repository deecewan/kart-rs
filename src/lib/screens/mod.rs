pub mod intro;
pub mod loading;
pub mod main_menu;
pub mod match_result;
pub mod race;
pub mod race_result;
pub mod select_character;

#[derive(Debug, PartialEq)]
pub enum Screen {
    Intro(intro::Intro),
    Loading(loading::Loading),
    MainMenu(main_menu::MainMenu),
    MatchResult(match_result::MatchResult),
    Race(race::Race),
    RaceResult(race_result::RaceResult),
    SelectCharacter(select_character::SelectCharacter),

    Unknown,
}
