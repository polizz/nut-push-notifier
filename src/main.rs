use tracing::{info, warn, debug};
use tracing_subscriber::EnvFilter;
use color_eyre::Report;

mod ups_status;
use ups_status::UpsStatus;

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
        ..
        // discharge_status_text, 
        // charge_status_text 
    } = args;

    let notifier = Notifier::new(gotify_url.as_str(), gotify_token.as_str());

    let interval = Duration::from_secs(nut_polling_secs);
    let auth = Some(Auth::new(nut_user, Some(nut_user_pass)));
    let config = ConfigBuilder::new()
        .with_host((nut_host, nut_host_port).try_into().unwrap_or_default())
        .with_auth(auth)
        .with_debug(false)
        .build();
    let mut conn = Connection::new(&config)?;

    let mut previous_status = UpsStatus::Startup;

    loop {
        let ups_variable = conn.get_var(&ups_name[..], &ups_variable[..])?;
        let status: UpsStatus = ups_variable.value().into();

        debug!(?status, ?ups_variable, "ups_variable");
        info!(%status);

        if previous_status == UpsStatus::Startup {
            previous_status = status;
            info!(%previous_status, "STARTUP WITH STATUS");

            if previous_status == UpsStatus::OnBattery {
                let notice_params = [("message", "INIT - UPS ONBATT - Discharging"), ("priority", "10")];
                notifier.send(&notice_params)
            }
        } else {
            if status == UpsStatus::Online && previous_status == UpsStatus::OnBattery {
                previous_status = UpsStatus::Online;
    
                info!("NOW ONLINE");

                let notice_params = [("message", "UPS ONLINE - Charging"), ("priority", "10")];
                notifier.send(&notice_params)
            } else if status == UpsStatus::OnBattery && previous_status == UpsStatus::Online {
                previous_status = UpsStatus::OnBattery;
    
                warn!("NOW ONBATT!!");

                let notice_params = [("message", "UPS ONBATT - Discharging"), ("priority", "10")];
                notifier.send(&notice_params)
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