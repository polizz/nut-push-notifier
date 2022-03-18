mod args;
// use args::*;

use std::convert::TryInto;
use std::{thread, time};
use rups::blocking::Connection;
use rups::{Auth, ConfigBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let nut_port = 3493;

    let auth = Some(Auth::new(username, password));
    let interval = time::Duration::from_secs(10);

    let config = ConfigBuilder::new()
        .with_host((nut_host, nut_port).try_into().unwrap_or_default())
        .with_auth(auth)
        .with_debug(false)
        .build();

    let mut conn = Connection::new(&config).unwrap();

    let mut previous_status = String::new();
    let on_battery: String = String::from("OB DISCHRG");
    let on_line: String = String::from("OL CHRG");

    loop {
        let ups_variable = conn.get_var("ups", "ups.status")?;
        let status = ups_variable.value();

        println!("Got status => {}", &status);

        if previous_status == "" {
            previous_status = status.clone();
            println!("INIT ON {}", &previous_status);

            if previous_status == on_battery {
                let notice_params = [("message", "INIT - UPS ONBATT - Discharging"), ("priority", "1")];
                notify(&notice_params)?;
            }
        } else {
            if status == on_line && previous_status == on_battery {
                previous_status = on_line.clone();
    
                println!("ONLINE");
                let notice_params = [("message", "UPS ONLINE - Charging"), ("priority", "1")];
                notify(&notice_params)?;
            } else if status == on_battery && previous_status == on_line {
                previous_status = on_battery.clone();
    
                println!("ONBATT");
                let notice_params = [("message", "UPS ONBATT - Discharging"), ("priority", "1")];
                notify(&notice_params)?;
            }
        }

        thread::sleep(interval);
    };
}

fn notify(notice_params: &[(&str, &str); 2]) -> Result<(), Box<dyn std::error::Error>> {

    let client = reqwest::blocking::Client::new();
    let resp = client.post(&notification_url)
        .form(&notice_params)
        .send()?;

    println!("Got response: {:?}", resp.status());

    Ok(())
}

// enumerate all ups and their vars

// conn.list_ups().unwrap()
// .iter()
// .for_each(|(name, desc)| {
//     println!("{}, {}", &name, &desc);

//     conn.list_vars(&name).unwrap()
//         .iter()
//         .filter(|variable| variable.to_string().starts_with("ups.status:"))
//         .for_each(|val| println!("\t- {}", &val))
//         // "ups.status: OL CHRG"
//         //  OB DISCHRG
// });