use std::{thread, time::Duration};
use color_eyre::Report;
use rups::blocking::Connection;
use tracing::{info, warn, debug};

use crate::utils::{Notifier, NoticeParam};
use crate::ups::{UpsState, UpsStatus};

static ONLINE: &str = "UPS ONLINE";
static CHARGE: &str = "UPS ONLINE - Charging";
static ON_BATT: &str = "UPS ONBATT - Discharging";

fn make_message_param<'local>(message: &'local str) -> Vec<NoticeParam> {
    vec![("message", message), ("priority", "10")]
}

pub fn execute(mut conn: Connection, addl_args: UpsStatusSpecs, notifier: impl Notifier) -> Result<(), Report> {
    let online_notice = make_message_param(ONLINE);
    let charge_notice = make_message_param(CHARGE);
    let on_battery_notice = make_message_param(ON_BATT);

    let UpsStatusSpecs {
        online_status_spec,
        discharge_status_spec, 
        charge_status_spec,
        ups_name, 
        nut_polling_secs, 
        ups_variable,
        verbose_online_status,
    } = addl_args;

    let mut ups_state = UpsState::new(
        online_status_spec,
        charge_status_spec,
        discharge_status_spec,
        verbose_online_status,
    );
    let interval = Duration::from_secs(nut_polling_secs);

    info!(%ups_state.status, "STARTUP WITH STATUS");

    loop {
        let ups_variable = conn.get_var(&ups_name, &ups_variable)?;
        let ups_variable_val = ups_variable.value();
        ups_state.update_status_from_str(ups_variable_val);

        debug!(?ups_state.status, ?ups_variable, "ups_variable");
        info!(%ups_state.status);

        if ups_state.is_state_changed() {
            match ups_state.status {
                UpsStatus::Charging => {
                    info!("NOW ONLINE, CHARGING");
                    notifier.send(&charge_notice);
                },
                UpsStatus::Online => {
                    info!("NOW ONLINE");
                    notifier.send(&online_notice);
                },
                UpsStatus::OnBattery => {
                    warn!("NOW ON BATTERY!");
                    notifier.send(&on_battery_notice);
                },
                UpsStatus::None(ref unknown_status_code) => {
                    info!(%unknown_status_code, "Encountered Unknown Status");
                    let message = format!("UPS Unknown Status Code - {}", &unknown_status_code);
                    notifier.send(&make_message_param(&message));
                },
                UpsStatus::Startup => (),
            }
        }
    
        thread::sleep(interval);
    };
}

pub struct UpsStatusSpecs<'main> {
    pub online_status_spec: &'main str,
    pub discharge_status_spec: &'main str,
    pub charge_status_spec: &'main str,
    pub ups_name: &'main str,
    pub ups_variable: &'main str,
    pub nut_polling_secs: u64,
    pub verbose_online_status: bool,
}
