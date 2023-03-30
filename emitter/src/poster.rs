use reqwest::blocking::Response;
use serde::Serialize;
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    time::Duration,
};

#[derive(Clone)]
pub struct Poster<T: Serialize>
where
    T: Clone,
{
    url: String,
    client: reqwest::blocking::Client,
    items: Arc<Mutex<VecDeque<T>>>,
}

impl<T> Poster<T>
where
    T: Serialize + Send + Clone + 'static,
{
    pub fn new() -> Self {
        let mut default_headers = reqwest::header::HeaderMap::new();
        default_headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        let client = reqwest::blocking::Client::builder()
            .default_headers(default_headers)
            .build()
            .expect("Couldn't create a request client");

        let url = std::env::var("KARTALYTICS_URL")
            .expect("KARTALYTICS_URL not set in the environment - it is required.");

        let items = Arc::new(Mutex::new(vec![].into()));

        let mut p = Poster { url, client, items };
        p.spawn_process_thread();

        return p;
    }

    pub fn queue(&self, data: T) {
        if let Ok(mut items) = self.items.lock() {
            println!("adding to the queue");
            items.push_back(data);
        }
    }

    fn spawn_process_thread(&mut self) {
        let mut clone = self.clone();

        std::thread::spawn(move || {
            let process_lock = Mutex::new(0);

            loop {
                // we only want to process one item at a time, so we lock on
                // processing
                let _lock = process_lock.lock().unwrap();
                // _but_ we want to be able to keep adding to the queue, so
                // we don't want to hold this lock for a long time
                let Some(item) = clone.items.lock()
                        .map_or(None, |mut items| items.pop_front())
                        else { continue; };

                match clone.process(item) {
                    Ok(res) => {
                        println!("Send Successful");
                        println!("Result: {res:?}");
                    }
                    Err(e) => {
                        eprintln!("Erroring sending request");
                        eprintln!("Error: {e:?}");
                    }
                }

                std::thread::sleep(Duration::from_millis(10));
            }
        });
    }

    fn process(&mut self, item: T) -> reqwest::Result<Response> {
        let json = serde_json::to_string_pretty(&item).unwrap();
        println!("processing {json}");
        self.client.post(&self.url).body(json).send()
    }
}

#[cfg(test)]
mod tests {
    use httpmock::prelude::*;
    use std::time::Duration;

    use super::Poster;

    #[test]
    fn it_sends_requests() {
        let server = MockServer::start();
        let mock_1 = server.mock(|when, then| {
            when.method("POST").path("/").body("\"Hello\"");
            then.status(201);
        });
        let mock_2 = server.mock(|when, then| {
            when.method("POST").path("/").body("\"World\"");
            then.status(201);
        });

        std::env::set_var("KARTALYTICS_URL", server.url("/"));
        let p = Poster::<String>::new();

        p.queue("Hello".into());
        p.queue("World".into());

        // this is much longer than _required_, because everything should be
        // done in ~20ms, but this feels like it'll reduce flakes
        std::thread::sleep(Duration::from_millis(100));

        mock_1.assert();
        mock_2.assert();
    }
}
