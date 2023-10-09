use crate::{ups::UpsStatus, NoticeParam};
// use reqwest::*;
use std::{thread, time::Duration};
use tracing::{info, span, warn, Level};
use ureq::Error;

pub struct GotifyNotifier {
    gotify_url: String,
    gotify_token: String,
    watch_receiver: tokio::sync::watch::Receiver<UpsStatus>,
}

static ONLINE: &str = "UPS ONLINE";
static CHARGE: &str = "UPS ONLINE - Charging";
static ON_BATT: &str = "UPS ONBATT - Discharging";

impl GotifyNotifier {
    pub fn new(url: String, token: String, rx: tokio::sync::watch::Receiver<UpsStatus>) -> Self {
        GotifyNotifier {
            gotify_url: url,
            gotify_token: token,
            watch_receiver: rx,
        }
    }

    fn make_message_param<T>(message: T) -> NoticeParam
    where
        T: Into<String>,
    {
        vec![
            ("message".to_string(), message.into()),
            ("priority".to_string(), "10".to_string()),
        ]
    }

    pub async fn listen(&mut self) {
        let online_notice = GotifyNotifier::make_message_param(ONLINE);
        let charge_notice = GotifyNotifier::make_message_param(CHARGE);
        let on_battery_notice = GotifyNotifier::make_message_param(ON_BATT);

        while self.watch_receiver.changed().await.is_ok() {
            let ups_status = *self.watch_receiver.borrow();
            let notice_params = ups_status;
            // debug!(?status_clone, ?ups_variable, "ups_variable");
            // info!(%ups_state.status);

            match ups_status {
                UpsStatus::Charging => {
                    info!("NOW ONLINE, CHARGING");
                    self.send_notice(&charge_notice);
                }
                UpsStatus::Online => {
                    info!("NOW ONLINE");
                    self.send_notice(&online_notice);
                }
                UpsStatus::OnBattery => {
                    warn!("NOW ON BATTERY!");
                    // notifier.send(&on_battery_notice);
                    self.send_notice(on_battery_notice);
                }
                UpsStatus::None(unknown_status_code) => {
                    info!(%unknown_status_code, "Encountered Unknown Status");
                    let message = format!("UPS Unknown Status Code - {}", unknown_status_code);
                    // notifier.send(&make_message_param(&message));
                    // let notice_param = vec![("message", message), ("priority", "10")];
                    let notice_param = vec![
                        ("message".to_string(), message),
                        ("priority".to_string(), "10".to_string()),
                    ];
                    self.send_notice(notice_param);
                }
                UpsStatus::Startup => (),
            }
        }
    }

    //     ("message".to_string(), msg),
    //     ("priority".to_string(), 10.to_string()),
    //     ("title".to_string(), title.to_string()),
    // ];
    // let client = reqwest::Client::new();
    // let res = client
    //     .post(GOTIFY_URL)
    //     .header("x-gotify-key", GOTIFY_KEY)
    //     .form(&params)
    //     .send()
    //     .await;
    // }

    fn send_notice(&self, &notice_params: NoticeParam) -> () {
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
                .send_form(&notice_params);

            ix = ix + 1;

            thread::sleep(Duration::from_secs(4))
        }
    }
}
