use argh::FromArgs;

#[derive(FromArgs, Debug)]
#[argh(description = "Commands")]
pub struct Top {
    #[argh(option, description = "nut host", short = 'h', default = "String::from(\"localhost\")")]
    pub nut_host: String,

    #[argh(option, description = "nut host port", short = 't', default = "3493")]
    pub nut_host_port: u16,

    #[argh(option, description = "nut user", short = 'j')]
    pub nut_user: String,

    #[argh(option, description = "nut user password", short = 'x')]
    pub nut_user_pass: String,

    #[argh(subcommand)]
    pub command: SubCommand,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
pub enum SubCommand {
    Watch(NotifyArgs),
    ListVars(ListArgs)
}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "listvars", description = "List variables from ups")]
pub struct ListArgs {}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "watch", description = "Notify on NUT state changes")]
pub struct NotifyArgs {
    #[argh(option,  description = "notification URL", short = 'u')]
    pub gotify_url: String,

    #[argh(option,  description = "notification token", short = 'p')]
    pub gotify_token: String,

    #[argh(option, description = "ups name", short = 'b', default = "String::from(\"ups\")")]
    pub ups_name: String,

    #[argh(option, description = "seconds to poll ups status", short = 'i', default = "60")]
    pub nut_polling_secs: u64,

    #[argh(option, description = "ups variable that holds online status", short = 'v', default = "String::from(\"ups.status\")")]
    pub ups_variable: String,

    #[argh(option, description = "discharging status text", short = 'd', default = "String::from(\"OB DISCHRG\")")]
    pub discharge_status_spec: String,

    #[argh(option, description = "charging status text", short = 'c', default = "String::from(\"OL CHRG\")")]
    pub charge_status_spec: String,
}