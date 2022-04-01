use std::{thread, time::Duration};
use tracing::{span, warn, Level};
use ureq::Error;

pub type NoticeParam<'local> = (&'local str, &'local str);

pub trait Notifier {
    fn new(url: String, token: String) -> Self;
    fn send(&self, notice_params: &Vec<NoticeParam>);
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

    fn send(&self, notice_params: &Vec<NoticeParam>) -> () {
        let notify_span = span!(Level::TRACE, "Notifying");
        let _notify_enter = notify_span.enter();

        let mut resp = ureq::post(&self.gotify_url)
            .set("x-gotify-key", &self.gotify_token)
            .send_form(&notice_params[..]);

        let retry_span = span!(Level::WARN, "Retrying Notification");
        let _retry_enter = retry_span.enter();

        let mut ix = 1;
        while let Err(Error::Status(_code, _response)) = resp {
            warn!(%ix, "Retrying notification");

            resp = ureq::post(&self.gotify_url)
                .set("x-gotify-key", &self.gotify_token)
                .send_form(&notice_params[..]);

            ix = ix + 1;

            thread::sleep(Duration::from_secs(4))
        }
    }
}
