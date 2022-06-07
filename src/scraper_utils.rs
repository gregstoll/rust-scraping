use std::sync::Mutex;
use std::time::Instant;

use lazy_static::lazy_static;
use reqwest::Error;

pub fn get_element_text(cell: &scraper::ElementRef) -> String {
    // The DOM allows multiple text nodes of an element, so join them all together.
    cell.text().collect::<Vec<_>>().join("").trim().to_string()
}

lazy_static! {
    static ref LAST_REQUEST_MUTEX: Mutex<Option<Instant>> = Mutex::new(None);
    static ref REQUEST_DELAY: std::time::Duration = std::time::Duration::from_millis(500);
}

// Do a request for the given URL, with a minimum time between requests
// to avoid overloading the server.
pub fn do_throttled_request(url: &str) -> Result<String, Error> {
    // GitHub Copilot wrote this whole method! (except for the REQUEST_DELAY variable)
    let mut last_request_mutex = LAST_REQUEST_MUTEX.lock().unwrap();
    let last_request = last_request_mutex.take();
    let now = Instant::now();
    if let Some(last_request) = last_request {
        let duration = now.duration_since(last_request);
        if duration < *REQUEST_DELAY {
            std::thread::sleep(*REQUEST_DELAY - duration);
        }
    }
    let response = reqwest::blocking::get(url)?;
    last_request_mutex.replace(now);
    Ok(response.text()?)
}