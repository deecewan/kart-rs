mod poster;

use chrono::{DateTime, Duration, Utc};
use serde::Serialize;
use std::sync::Mutex;

#[derive(Debug, Serialize, Clone)]
struct Event<T: Serialize> {
    event_type: String,

    data: T,

    timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Clone)]
struct Emittable<T: Serialize> {
    events: Vec<Event<T>>,
}

struct Inner<T: Serialize + Clone + std::cmp::PartialEq> {
    last_emitted_data: Option<T>,
    last_emitted_at: DateTime<Utc>,
}

pub enum Mode {
    Real, // TODO: what is this name???
    Debug,
}

pub struct Emit<T: Serialize + Clone + std::cmp::PartialEq> {
    inner: Mutex<Inner<T>>,
    emitter: Option<poster::Poster<Emittable<T>>>,
}

impl<T> Emit<T>
where
    for<'a> T: Serialize + Clone + std::cmp::PartialEq + Send + 'a,
{
    pub fn new<'a>(mode: Mode) -> Self {
        let inner = Inner {
            last_emitted_data: None,
            last_emitted_at: Utc::now()
                .checked_sub_signed(Duration::milliseconds(501))
                .unwrap(),
        };

        let emitter = match mode {
            Mode::Real => Some(poster::Poster::new()),
            Mode::Debug => None,
        };

        Emit {
            emitter,
            inner: Mutex::new(inner),
        }
    }

    pub fn emit(&self, event_type: &str, data: &T) {
        if self.skip_emit(data) {
            println!("Skipping the emit");
            return;
        }

        let now = Utc::now();

        let event = Event {
            data: data.clone(),
            event_type: event_type.into(),
            timestamp: now,
        };

        let emittable = Emittable {
            events: vec![event],
        };

        self.update_inner(data.clone());

        match &self.emitter {
            Some(emitter) => {
                emitter.queue(emittable.clone());
            }
            None => {}
        };
    }

    fn skip_emit(&self, data: &T) -> bool {
        // if the screens are different _or_ we've exceeded the timestamp, we
        // need to try again
        if self.exceeded_timestamp() {
            println!("can't skip because longer than 0.5s");
            return false;
        }

        if self.data_is_different_from_last_sent(data) {
            println!("can't skip because the screens are different");
            return false;
        }

        return true;
    }

    fn data_is_different_from_last_sent(&self, data: &T) -> bool {
        let inner = self.inner.lock().expect("failed to lock the mutex");

        match &inner.last_emitted_data {
            // if we have never emitted a screen, we want to emit this one
            None => true,
            Some(old_data) => data != old_data,
        }
    }

    // we _always_ send every 0.5s.
    fn exceeded_timestamp(&self) -> bool {
        let inner = self.inner.lock().expect("failed to lock the mutex");

        let now = Utc::now();
        let diff = now.signed_duration_since(inner.last_emitted_at);

        diff.num_milliseconds() > 500
    }

    fn update_inner(&self, data: T) {
        let new_inner = Inner {
            last_emitted_at: Utc::now(),
            last_emitted_data: Some(data),
        };

        let mut inner = self.inner.lock().expect("failed to lock the mutex");
        *inner = new_inner;
    }
}

#[cfg(test)]
mod tests {
    use httpmock::prelude::*;
    use std::time::Duration;

    use super::*;
    // i hate these tests but something isn't right
    #[test]
    fn it_queues_requests() {
        let server = MockServer::start();
        let mock_1 = server.mock(|when, then| {
            when.method("POST")
                .path("/")
                .body_contains("\"data\": \"First\"")
                .body_contains("\"event_type\": \"event type\"");
            then.status(201);
        });
        let mock_2 = server.mock(|when, then| {
            when.method("POST")
                .path("/")
                .body_contains("\"data\": \"Second\"")
                .body_contains("\"event_type\": \"event type\"");
            then.status(201);
        });

        std::env::set_var("KARTALYTICS_URL", server.url("/"));

        let emitter = Emit::<String>::new(Mode::Real);

        for _ in 0..100 {
            let data: String = "First".into();
            emitter.emit("event type", &data);
            // let data: String = "Second".into();
            // emitter.emit("event type", &data);
        }
        for _ in 0..100 {
            let data: String = "Second".into();
            emitter.emit("event type", &data);
            // let data: String = "Second".into();
            // emitter.emit("event type", &data);
        }
        for _ in 0..100 {
            let data: String = "First".into();
            emitter.emit("event type", &data);
            // let data: String = "Second".into();
            // emitter.emit("event type", &data);
        }
        for _ in 0..100 {
            let data: String = "Second".into();
            emitter.emit("event type", &data);
            // let data: String = "Second".into();
            // emitter.emit("event type", &data);
        }

        // this is much longer than _required_, because everything should be
        // done in ~20ms, but this feels like it'll reduce flakes
        std::thread::sleep(Duration::from_millis(10));

        println!("hits: {}", mock_1.hits());
        mock_1.assert_hits(2);
        mock_2.assert_hits(2);
    }
}
