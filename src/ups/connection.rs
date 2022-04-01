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
