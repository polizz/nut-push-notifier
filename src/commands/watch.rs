use std::{thread, time::Duration};
use rups::blocking::Connection;
use rups::{Auth, ConfigBuilder};
use color_eyre::Report;
use tracing::{info, warn, debug};

use crate::notify::*;
use crate::args::NotifyArgs;
use crate::{UpsState, UpsStatus};

use super::{Command, CommandArgs};

pub struct Watch;

impl Command<NotifyArgs> for Watch {
    fn execute(args: impl CommandArgs<NotifyArgs>) -> Result<(), Report> {
        let NotifyArgs {
            nut_host, 
            nut_host_port, 
            nut_user, 
            nut_user_pass, 
            gotify_url, 
            gotify_token, 
            ups_name, 
            nut_polling_secs, 
            ups_variable,
            discharge_status_spec, 
            charge_status_spec, 
        } = args.get_args();
        let mut ups_state = UpsState::new(charge_status_spec, discharge_status_spec);
        let notifier = Notifier::new(gotify_url.as_str(), gotify_token.as_str());
        let interval = Duration::from_secs(nut_polling_secs);
        let auth = Some(Auth::new(nut_user, Some(nut_user_pass)));
        let config = ConfigBuilder::new()
            .with_host((nut_host, nut_host_port).try_into().unwrap_or_default())
            .with_auth(auth)
            .with_debug(false)
            .build();
        let mut conn = Connection::new(&config)?;
      
        info!(%ups_state.status, "STARTUP WITH STATUS");
      
        loop {
            let ups_variable = conn.get_var(&ups_name[..], &ups_variable[..])?;
            ups_state.update_status_from_str(ups_variable.value());
      
            debug!(?ups_state.status, ?ups_variable, "ups_variable");
            info!(%ups_state.status);
      
            if ups_state.is_state_changed() {
                match ups_state.status.clone() {
                    UpsStatus::OnBattery => {
                        warn!("NOW ONBATT!!");
                        let notice_params = [("message", "UPS ONBATT - Discharging"), ("priority", "10")];
                        notifier.send(&notice_params)
                    },
                    UpsStatus::Online => {
                        info!("NOW ONLINE");
                        let notice_params = [("message", "UPS ONLINE - Charging"), ("priority", "10")];
                        notifier.send(&notice_params)
                    },
                    UpsStatus::None(unknown_status_code) => {
                        info!(%unknown_status_code, "Encountered Unknown Status");
                        let status = format!("UPS Unknown Status Code - {}", &unknown_status_code);
                        let notice_params = [("message", status.as_str()), ("priority", "10")];
                        notifier.send(&notice_params)
                    },
                    UpsStatus::Startup => (),
                }
            }
      
            thread::sleep(interval);
        };
      }
}

#[cfg(test)]
mod tests {
    #[test]
    fn smoke() {
        
    }
}