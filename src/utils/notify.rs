use crate::{ups::UpsStatus, NoticeParam, StatusEvent};
// use reqwest::*;
use tracing::{info, span, warn, Level};
use ureq::Error;

pub struct GotifyNotifier {
    gotify_url: String,
    gotify_token: String,
    watch_receiver: tokio::sync::watch::Receiver<StatusEvent>,
}

static ONLINE: &str = "UPS ONLINE";
static CHARGE: &str = "UPS ONLINE - Charging";
static ON_BATT: &str = "UPS ONBATT - Discharging";

impl GotifyNotifier {
    pub fn new(url: String, token: String, rx: tokio::sync::watch::Receiver<StatusEvent>) -> Self {
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
            let state_event = self.watch_receiver.borrow();

            if state_event.changed {
                match state_event.ups_status {
                    UpsStatus::Charging => {
                        info!("NOW ONLINE, CHARGING");
                        self.send_notice(&charge_notice).await;
                    }
                    UpsStatus::Online => {
                        info!("NOW ONLINE");
                        self.send_notice(&online_notice).await;
                    }
                    UpsStatus::OnBattery => {
                        warn!("NOW ON BATTERY!");
                        self.send_notice(&on_battery_notice).await;
                    }
                    UpsStatus::None(ref unknown_status_code) => {
                        info!(%unknown_status_code, "Encountered Unknown Status");
                        let message = format!("UPS Unknown Status Code - {}", unknown_status_code);
                        let notice_param = vec![
                            ("message".to_string(), message),
                            ("priority".to_string(), "10".to_string()),
                        ];
                        self.send_notice(&notice_param).await;
                    }
                    UpsStatus::Startup => (),
                }
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

    async fn send_notice(&self, notice_params: &NoticeParam) -> () {
        let notify_span = span!(Level::TRACE, "Notifying");
        let _notify_enter = notify_span.enter();

        let slice_params = notice_params
            .iter()
            .map(|(p1, p2)| (p1 as &str, p2 as &str))
            .collect::<Vec<(&str, &str)>>();

        let mut resp = ureq::post(&self.gotify_url)
            .set("x-gotify-key", &self.gotify_token)
            .send_form(&slice_params.as_slice());

        let retry_span = span!(Level::WARN, "Retrying Notification");
        let _retry_enter = retry_span.enter();

        let mut ix = 1;
        while let Err(Error::Status(_code, _response)) = resp {
            warn!(%ix, "Retrying notification");

            resp = ureq::post(&self.gotify_url)
                .set("x-gotify-key", &self.gotify_token)
                .send_form(&slice_params.as_slice());

            ix = ix + 1;

            tokio::time::sleep(std::time::Duration::from_millis(1000 * 4)).await;
        }
    }
}
