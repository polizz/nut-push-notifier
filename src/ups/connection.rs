use rups::blocking::Connection;
use rups::{Auth, ConfigBuilder};
use super::super::Top;

pub fn make_rups_connection(
    Top {
        nut_host,
        nut_host_port,
        nut_user,
        nut_user_pass,
        command: _,
    }: &Top,
) -> Connection {
    let auth = Some(Auth::new(nut_user.clone(), Some(nut_user_pass.clone())));
    let config = ConfigBuilder::new()
        .with_host((nut_host.clone(), nut_host_port.clone()).try_into().unwrap_or_default())
        .with_auth(auth)
        .with_debug(false)
        .build();
    Connection::new(&config).unwrap()
}

pub trait UpsConnection {
    fn get_var(&mut self, ups_name: &str, variable: &str) -> rups::Result<rups::Variable>;
    fn list_ups(&mut self) -> rups::Result<Vec<(String, String)>>;
    fn list_vars(&mut self, ups_name: &str) -> rups::Result<Vec<rups::Variable>>;
}

pub struct RupsConnection {
    connection: rups::blocking::Connection,
}

impl RupsConnection {
    pub fn new(connection: rups::blocking::Connection) -> Self {
        Self { connection }
    }
}

impl UpsConnection for RupsConnection {
    fn get_var(&mut self, ups_name: &str, variable: &str) -> rups::Result<rups::Variable> {
        self.connection.get_var(&ups_name, &variable)
    }

    fn list_ups(&mut self) -> rups::Result<Vec<(String, String)>> {
        self.connection.list_ups()
    }

    fn list_vars(&mut self, ups_name: &str) -> rups::Result<Vec<rups::Variable>> {
        self.connection.list_vars(&ups_name)
    }
}