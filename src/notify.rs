use std::{thread, time::Duration};
use tracing::{span, warn, Level};

pub trait Notifier {
    fn new(url: String, token: String) -> Self;
    fn send(&self, notice_params: &[(&str, &str); 2]);
}

pub struct GotifyNotifier {
    gotify_url: String,
    gotify_token: String,
}

impl Notifier for GotifyNotifier {
    fn new(url: String, token: String) -> Self {
        GotifyNotifier {
            gotify_url: url,
            gotify_token: token,
        }
    }

    fn send(&self, notice_params: &[(&str, &str); 2]) -> () {
        let notify_span = span!(Level::TRACE, "Notifying");
        let _notify_enter = notify_span.enter();

        let client = reqwest::blocking::Client::new();
        let mut resp = client
            .post(&self.gotify_url)
            .form(&notice_params)
            .header("x-gotify-key", &self.gotify_token)
            .send();

        let retry_span = span!(Level::WARN, "Retrying Notification");
        let _retry_enter = retry_span.enter();

        let mut ix = 1;
        while let Err(_) = resp {
            warn!(%ix, "Retrying notification");

            resp = client
                .post(&self.gotify_url)
                .form(&notice_params)
                .header("x-gotify-key", &self.gotify_token)
                .send();
            ix = ix + 1;

            thread::sleep(Duration::from_secs(4))
        }
    }
}

// impl<'a> GotifyNotifier<'a> {
//     pub fn new(url: &'a str, token: &'a str) -> Self {
//         GotifyNotifier {
//             gotify_url: url,
//             gotify_token: token,
//         }
//     }

//     pub fn send(&self, notice_params: &[(&str, &str); 2]) -> () {
//         let notify_span = span!(Level::TRACE, "Notifying");
//         let _notify_enter = notify_span.enter();

//         let client = reqwest::blocking::Client::new();
//         let mut resp = client
//             .post(self.gotify_url)
//             .form(&notice_params)
//             .header("x-gotify-key", self.gotify_token)
//             .send();

//         let retry_span = span!(Level::WARN, "Retrying Notification");
//         let _retry_enter = retry_span.enter();

//         let mut ix = 1;
//         while let Err(_) = resp {
//             warn!(%ix, "Retrying notification");

//             resp = client
//                 .post(self.gotify_url)
//                 .form(&notice_params)
//                 .header("x-gotify-key", self.gotify_token)
//                 .send();
//             ix = ix + 1;

//             thread::sleep(Duration::from_secs(4))
//         }
//     }
// }
