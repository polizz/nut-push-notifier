use std::{thread, time::Duration};
use color_eyre::Report;
use rups::blocking::Connection;
use tracing::{info, warn, debug};

use crate::utils::{Notifier, NoticeParam};
use crate::ups::{UpsState, UpsStatus};

pub fn execute(mut conn: Connection, addl_args: UpsStatusSpecs, notifier: impl Notifier) -> Result<(), Report> {
    let make_message_param = |message| -> Vec<NoticeParam> { vec![("message".into(), message), ("priority".into(), "10".into())] };
    let on_battery_notice = make_message_param("UPS ONBATT - Discharging".to_string());
    let online_notice = make_message_param("UPS ONLINE - Charging".to_string());

    let UpsStatusSpecs {
        discharge_status_spec, 
        charge_status_spec,
        ups_name, 
        nut_polling_secs, 
        ups_variable,
    } = addl_args;
    let mut ups_state = UpsState::new(charge_status_spec, discharge_status_spec);
    let interval = Duration::from_secs(nut_polling_secs);

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
                    notifier.send(&on_battery_notice);
                },
                UpsStatus::Online => {
                    info!("NOW ONLINE");
                    notifier.send(&online_notice);
                },
                UpsStatus::None(unknown_status_code) => {
                    info!(%unknown_status_code, "Encountered Unknown Status");
                    let message = format!("UPS Unknown Status Code - {}", &unknown_status_code);
                    notifier.send(&make_message_param(message))
                },
                UpsStatus::Startup => (),
            }
        }
    
        thread::sleep(interval);
    };
}

pub struct UpsStatusSpecs {
    pub discharge_status_spec: String,
    pub charge_status_spec: String,
    pub ups_name: String,
    pub nut_polling_secs: u64,
    pub ups_variable: String,
}
