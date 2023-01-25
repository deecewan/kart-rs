use std::sync::Mutex;

use crate::screens::Screen;
use chrono::{DateTime, Duration, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Emittable<'a> {
    event_type: &'static str,

    #[serde(rename = "data")]
    screen: &'a Screen,

    timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
struct EmittableWrapper<'a> {
    events: Vec<Emittable<'a>>,
}

struct EmitInner {
    last_emitted_screen: Option<Screen>,
    last_emitted_at: DateTime<Utc>,
}

pub struct Emit {
    inner: Mutex<EmitInner>,
    client: reqwest::blocking::Client,
}

impl Emit {
    pub fn new() -> Self {
        let inner = EmitInner {
            last_emitted_screen: None,
            last_emitted_at: Utc::now()
                .checked_sub_signed(Duration::milliseconds(501))
                .unwrap(),
        };

        let mut default_headers = reqwest::header::HeaderMap::new();
        default_headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        let client = reqwest::blocking::Client::builder()
            .default_headers(default_headers)
            .build()
            .expect("Couldn't create a request client");

        Emit {
            inner: Mutex::new(inner),
            client,
        }
    }

    pub fn emit(&self, screen: &Option<Screen>) {
        let Some(screen) = screen else { return; };

        if self.skip_emit(screen) {
            println!("Skipping the emit");
            return;
        }

        let now = Utc::now();

        let event_type = match screen {
            Screen::Intro(_) => "intro_screen",
            Screen::Loading(_) => "loading_screen",
            Screen::MainMenu(_) => "main_menu_screen",
            Screen::Race(_) => "race_screen",
            Screen::SelectCharacter(_) => "select_character_screen",
            Screen::MatchResult(_) => "match_result_screen",
            Screen::RaceResult(_) => "race_result_screen",

            // Skip Unknown screens - no need to emit
            Screen::Unknown => return,
        };

        let emittable = EmittableWrapper {
            events: vec![Emittable {
                screen,
                event_type,
                timestamp: now,
            }],
        };

        self.update_inner(screen.clone());

        let json = serde_json::to_string_pretty(&emittable).unwrap();

        let posted = self
            .client
            .post("http://127.0.0.1:3000/api/kartalytics/ingest")
            .body(json)
            .send()
            .unwrap();

        dbg!(posted.text().unwrap());

        println!("{}", serde_json::to_string_pretty(&emittable).unwrap());
    }

    fn update_inner(&self, screen: Screen) {
        let new_inner = EmitInner {
            last_emitted_at: Utc::now(),
            last_emitted_screen: Some(screen),
        };

        let mut inner = self.inner.lock().expect("failed to lock the mutex");
        *inner = new_inner;
    }

    fn skip_emit(&self, screen: &Screen) -> bool {
        // if the screens are different _or_ we've exceeded the timestamp, we
        // need to try again
        if self.exceeded_timestamp() {
            println!("can't skip because longer than 0.5s");
            return false;
        }

        if self.screen_is_different_from_last_sent(screen) {
            println!("can't skip because the screens are different");
            return false;
        }

        return true;
    }

    fn screen_is_different_from_last_sent(&self, screen: &Screen) -> bool {
        let inner = self.inner.lock().expect("failed to lock the mutex");

        match &inner.last_emitted_screen {
            // if we have never emitted a screen, we want to emit this one
            None => true,
            Some(old_screen) => screen != old_screen,
        }
    }

    // we _always_ send every 0.5s.
    fn exceeded_timestamp(&self) -> bool {
        let inner = self.inner.lock().expect("failed to lock the mutex");

        let now = Utc::now();
        let diff = now.signed_duration_since(inner.last_emitted_at);

        println!("diff was {}", diff);

        diff.num_milliseconds() > 500
    }
}
