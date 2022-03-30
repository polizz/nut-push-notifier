use std::{thread, time::Duration};
use color_eyre::Report;
use tracing::{info, warn, debug};

use crate::utils::Notifier;
use crate::ups::{UpsConnection, UpsState, UpsStatus};

pub fn execute(mut conn: impl UpsConnection, addl_args: UpsStatusSpecs, notifier: impl Notifier) -> Result<(), Report> {
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

pub struct UpsStatusSpecs {
    pub discharge_status_spec: String,
    pub charge_status_spec: String,
    pub ups_name: String,
    pub nut_polling_secs: u64,
    pub ups_variable: String,
}

// #[cfg(test)]
// mod tests {
    
//     struct TestArgs {

//     }

//     impl CommandArgs for TestArgs {
//         fn get_args(self) -> TestArgs {
//             self
//         }
//     } 

//     #[test]
//     fn smoke() {
//         let 

//         Watch::execute(args, notifier);
//     }
// }