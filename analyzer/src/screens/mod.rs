use serde::Serialize;

pub mod intro;
pub mod loading;
pub mod main_menu;
pub mod match_result;
pub mod race;
pub mod race_result;
pub mod select_character;

#[derive(Debug, PartialEq, Serialize, Clone)]
#[serde(untagged)]
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

impl Screen {
    pub fn event_type(&self) -> &'static str {
        match self {
            Screen::Intro(_) => "intro_screen",
            Screen::Loading(_) => "loading_screen",
            Screen::MainMenu(_) => "main_menu_screen",
            Screen::Race(_) => "race_screen",
            Screen::SelectCharacter(_) => "select_character_screen",
            Screen::MatchResult(_) => "match_result_screen",
            Screen::RaceResult(_) => "race_result_screen",

            // Skip Unknown screens - no need to emit
            Screen::Unknown => "unknown_screen",
        }
    }
}
