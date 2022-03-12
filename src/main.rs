mod args;
// use args::*;

use rups::blocking::Connection;
use rups::{Auth, ConfigBuilder};
use std::convert::TryInto;

fn main() {
    let port = 3493;

    let auth = Some(Auth::new(username, password));

    let config = ConfigBuilder::new()
        .with_host((host, port).try_into().unwrap_or_default())
        .with_auth(auth)
        .with_debug(false)
        .build();

    let mut conn = Connection::new(&config).unwrap();

    conn.list_ups().unwrap()
        .iter()
        .for_each(|(name, desc)| {
            println!("{}, {}", &name, &desc);

            conn.list_vars(&name).unwrap().iter()
                .for_each(|val| println!("\t- {}", &val))
        })

    // conn.list_vars()
}
