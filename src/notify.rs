use std::{thread, time::Duration};
use tracing::{span, warn, Level};

pub struct Notifier<'a> {
    gotify_url: &'a str,
    gotify_token: &'a str,
}

impl<'a> Notifier<'a> {
    pub fn new(url: &'a str, token: &'a str) -> Self {
        Notifier {
            gotify_url: url,
            gotify_token: token,
        }
    }

    pub fn send(&self, notice_params: &[(&str, &str); 2]) -> () {
        let notify_span = span!(Level::TRACE, "Notifying");
        let _notify_enter = notify_span.enter();

        let client = reqwest::blocking::Client::new();
        let mut resp = client
            .post(self.gotify_url)
            .form(&notice_params)
            .header("x-gotify-key", self.gotify_token)
            .send();

        let retry_span = span!(Level::WARN, "Retrying Notification");
        let _retry_enter = retry_span.enter();

        let mut ix = 1;
        while let Err(_) = resp {
            warn!(%ix, "Retrying notification");

            resp = client
                .post(self.gotify_url)
                .form(&notice_params)
                .header("x-gotify-key", self.gotify_token)
                .send();
            ix = ix + 1;

            thread::sleep(Duration::from_secs(4))
        }
    }
}
