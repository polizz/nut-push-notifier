use color_eyre::Report;
use rups::blocking::Connection;
use rups::{Auth, ConfigBuilder};

use crate::args::ListArgs;
use crate::notify::*;

use super::command::{Command, CommandArgs};

pub struct List;

impl Command<ListArgs> for List {
    fn execute(args: impl CommandArgs<ListArgs>, _na: impl Notifier) -> Result<(), Report> {
        let ListArgs {
            nut_host,
            nut_host_port,
            nut_user,
            nut_user_pass,
        } = args.get_args();
        let auth = Some(Auth::new(nut_user, Some(nut_user_pass)));
        let config = ConfigBuilder::new()
            .with_host((nut_host, nut_host_port).try_into().unwrap_or_default())
            .with_auth(auth)
            .with_debug(false)
            .build();
        let mut conn = Connection::new(&config)?;
    
        conn.list_ups()?.iter().for_each(|(name, desc)| {
            println!("UPS Name: {}, Description: {}", &name, &desc);
    
            conn.list_vars(&name)
                .unwrap()
                .iter()
                .for_each(|val| println!("\t- {}", &val))
        });
    
        Ok(())
    }
}
