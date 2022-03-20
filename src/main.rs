mod args;
use args::{Top, ListArgs, NotifyArgs }; //NotifyArgs

use std::convert::TryInto;
use std::{thread, time};
use chrono::Local;
use rups::blocking::Connection;
use rups::{Auth, ConfigBuilder};

fn run_list_command(ListArgs { nut_host, nut_host_port, nut_user, nut_user_pass }: ListArgs)
        -> Result<(), Box<dyn std::error::Error>> {
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
        println!("UPS Name: {}, Description: {}", &name, &desc);

        conn.list_vars(&name).unwrap()
            .iter()
            .for_each(|val| println!("\t- {}", &val))
    });

    Ok(())
}

fn watch(NotifyArgs { nut_host, nut_host_port, nut_user, nut_user_pass, gotify_url, ups_name, nut_polling_secs, ups_variable, discharge_status_text, charge_status_text }: NotifyArgs)
        -> Result<(), Box<dyn std::error::Error>> {
    let interval = time::Duration::from_secs(nut_polling_secs);
    let auth = Some(Auth::new(nut_user, Some(nut_user_pass)));
    let config = ConfigBuilder::new()
        .with_host((nut_host, nut_host_port).try_into().unwrap_or_default())
        .with_auth(auth)
        .with_debug(false)
        .build();
    let mut conn = Connection::new(&config)?;

    let mut previous_status = String::new();

    loop {
        let ups_variable = conn.get_var(&ups_name[..], &ups_variable[..])?;
        let status = ups_variable.value();

        println!("{} \t=> Got status => {}", Local::now().format("%Y-%m-%d][%H:%M:%S"), &status);

        if previous_status == "" {
            previous_status = status.clone();
            println!("INIT ON {}", &previous_status);

            if previous_status == discharge_status_text {
                let notice_params = [("message", "INIT - UPS ONBATT - Discharging"), ("priority", "1")];
                notify(&gotify_url, &notice_params)?;
            }
        } else {
            if status == charge_status_text && previous_status == discharge_status_text {
                previous_status = charge_status_text.clone();
    
                println!("ONLINE");
                let notice_params = [("message", "UPS ONLINE - Charging"), ("priority", "1")];
                notify(&gotify_url, &notice_params)?;
            } else if status == discharge_status_text && previous_status == charge_status_text {
                previous_status = discharge_status_text.clone();
    
                println!("ONBATT!!");
                let notice_params = [("message", "UPS ONBATT - Discharging"), ("priority", "1")];
                notify(&gotify_url, &notice_params)?;
            }
        }

        thread::sleep(interval);
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let top: Top = argh::from_env();

    match top.command {
        args::SubCommand::ListVars(args) => run_list_command(args),
        args::SubCommand::Watch(args) => watch(args),
    }
}

fn notify(gotify_url: &str, notice_params: &[(&str, &str); 2]) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let _ = client.post(gotify_url)
        .form(&notice_params)
        .send()?;

    Ok(())
}