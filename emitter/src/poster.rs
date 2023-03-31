use reqwest::blocking::Response;
use serde::Serialize;
use std::sync::mpsc;

#[derive(Clone)]
pub struct Poster<T: Serialize>
where
    T: Clone,
{
    url: String,
    client: reqwest::blocking::Client,
    sender: mpsc::SyncSender<T>,
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

        let (sender, receiver) = mpsc::sync_channel::<T>(100);

        let mut p = Poster {
            url,
            client,
            sender,
        };

        p.spawn_process_thread(receiver);

        return p;
    }

    pub fn queue(&self, data: T) {
        match self.sender.send(data) {
            Ok(_) => { /* do nothing */ }
            Err(e) => {
                eprintln!("Failed to queue: {e:?}");
            }
        }
    }

    fn spawn_process_thread(&mut self, rx: mpsc::Receiver<T>) {
        let mut clone = self.clone();

        std::thread::spawn(move || {
            for item in rx.iter() {
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
        // done in ~1ms, but this feels like it'll reduce flakes
        std::thread::sleep(Duration::from_millis(10));

        mock_1.assert();
        mock_2.assert();
    }
}
