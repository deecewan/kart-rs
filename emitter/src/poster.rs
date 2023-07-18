use log::{error, info};
use log_err::LogErrResult;
use reqwest::Response;
use serde::Serialize;
use std::sync::mpsc;
use tokio::runtime::Runtime;

#[derive(Clone)]
pub struct Poster<T: Serialize>
where
    T: Clone,
{
    url: String,
    client: reqwest::Client,
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

        let client = reqwest::Client::builder()
            .default_headers(default_headers)
            .build()
            .log_expect("Couldn't create a request client");

        let url = std::env::var("KARTALYTICS_URL")
            .log_expect("KARTALYTICS_URL not set in the environment - it is required.");

        let (sender, receiver) = mpsc::sync_channel::<T>(2000);

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
                error!("Failed to queue: {e:?}");
            }
        }
    }

    fn spawn_process_thread(&mut self, rx: mpsc::Receiver<T>) {
        let clone = self.clone();

        std::thread::spawn(move || {
            let rt = Runtime::new().log_expect("Couldn't create a tokio runtime");

            rt.block_on(async move {
                for item in rx.iter() {
                    let mut pool_clone = clone.clone();
                    tokio::spawn(async move {
                        match pool_clone.process(item).await {
                            Ok(_) => {}
                            Err(e) => {
                                error!("error sending request: {e:?}");
                            }
                        }
                    });
                }
            });
        });
    }

    async fn process(&mut self, item: T) -> reqwest::Result<Response> {
        let json = serde_json::to_string_pretty(&item).unwrap();
        let start = chrono::Utc::now();
        let sent_json = json.clone();

        info!("sending: {json}");

        let res = self.client.post(&self.url).body(sent_json).send().await;

        info!("result: {res:?}");

        let end = chrono::Utc::now();

        let delta = end - start;

        info!("Send took {delta:?}");

        return res;
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
            when.method(httpmock::Method::POST)
                .path("/")
                .body("\"Hello\"");
            then.status(201);
        });
        let mock_2 = server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path("/")
                .body("\"World\"");
            then.status(201);
        });

        let server_url = server.url("/");
        std::env::set_var("KARTALYTICS_URL", server_url);
        let p = Poster::<String>::new();

        p.queue("Hello".into());
        p.queue("World".into());

        // this is much longer than _required_, because everything should be
        // done in ~1ms, but this feels like it'll reduce flakes
        std::thread::sleep(Duration::from_millis(100));

        mock_1.assert();
        mock_2.assert();
    }

    #[test]
    fn it_sends_heaps() {
        let server = MockServer::start();
        let mock_1 = server.mock(|when, then| {
            when.method("POST").path("/");
            then.status(201);
        });

        let server_url = server.url("/");
        std::env::set_var("KARTALYTICS_URL", server_url);
        let p = Poster::<i32>::new();

        for i in 0..100 {
            p.queue(i);
        }

        // this is much longer than _required_, because everything should be
        // done in ~1ms, but this feels like it'll reduce flakes
        std::thread::sleep(Duration::from_millis(1000));

        mock_1.assert_hits(10000);

        println!("hits: {}", mock_1.hits());
    }
}
