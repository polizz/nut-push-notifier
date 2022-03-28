use tracing::{info, warn, debug};
use tracing_subscriber::EnvFilter;
use color_eyre::Report;

mod ups_state;
use ups_state::*;

mod args;
use args::{Top, ListArgs, NotifyArgs }; //NotifyArgs

mod notify;
use notify::Notifier;

use std::convert::TryInto;
use std::{thread, time::Duration};
use rups::blocking::Connection;
use rups::{Auth, ConfigBuilder};

fn run_list_command(args: ListArgs) -> Result<(), Report> {
    let ListArgs { nut_host, nut_host_port, nut_user, nut_user_pass } = args;
    let auth = Some(Auth::new(nut_user, Some(nut_user_pass)));
    let config = ConfigBuilder::new()
        .with_host((nut_host, nut_host_port).try_into().unwrap_or_default())
        .with_auth(auth)
        .with_debug(false)
        .build();
    let mut conn = Connection::new(&config)?;

    conn.list_ups()?
        .iter()
        .for_each(|(name, desc)| {
            info!("UPS Name: {}, Description: {}", &name, &desc);

            conn.list_vars(&name).unwrap()
                .iter()
                .for_each(|val| info!("\t- {}", &val))
        });

    Ok(())
}

fn watch(args: NotifyArgs) -> Result<(), Report> {
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
    } = args;
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

    // let mut previous_status = ups_state.status;
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

fn main() -> Result<(), Report> {
    if std::env::var("RUST_BACKTRACE").is_err() {
        std::env::set_var("RUST_BACKTRACE", "1")
    }
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let top: Top = argh::from_env();
    
    match top.command {
        args::SubCommand::ListVars(args) => run_list_command(args),
        args::SubCommand::Watch(args) => watch(args),
    }
}