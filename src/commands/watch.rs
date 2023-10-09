use color_eyre::Report;
use rups::blocking::Connection;
use tracing::{debug, info, warn};

use crate::ups::{UpsState, UpsStatus};

pub async fn execute(
    mut conn: Connection,
    addl_args: UpsStatusSpecs,
    watch_sender: tokio::sync::watch::Sender<UpsStatus>,
) -> Result<(), Report> {
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
    info!(%ups_state.status, "STARTUP WITH STATUS");

    loop {
        let ups_variable = conn.get_var(&ups_name, &ups_variable)?;
        let ups_variable_val = ups_variable.value();
        ups_state.update_status_from_str(ups_variable_val);

        if ups_state.is_state_changed() {
            watch_sender.send(ups_state.status.clone());
        }

        tokio::time::sleep(std::time::Duration::from_millis(1000 * nut_polling_secs)).await;
    }
}

pub struct UpsStatusSpecs {
    pub online_status_spec: String,
    pub discharge_status_spec: String,
    pub charge_status_spec: String,
    pub ups_name: String,
    pub ups_variable: String,
    pub nut_polling_secs: u64,
    pub verbose_online_status: bool,
}
