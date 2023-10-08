use color_eyre::Report;
use tracing_subscriber::EnvFilter;

mod ups;
use ups::*;

mod utils;
use utils::*;

mod commands;
use commands::*;

mod ws;
use ws::ws_server;

#[tokio::main]
async fn main() -> Result<(), Report> {
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
    let rups_connection = make_rups_connection(&top);

    if let SubCommand::Watch(args) = top.command {
        let addl_args = UpsStatusSpecs {
            online_status_spec: args.online_status_spec,
            discharge_status_spec: args.discharge_status_spec,
            charge_status_spec: args.charge_status_spec,
            ups_name: args.ups_name,
            ups_variable: args.ups_variable,
            nut_polling_secs: args.nut_polling_secs,
            verbose_online_status: args.verbose_online_status,
        };

        let notifier = GotifyNotifier::new(args.gotify_url, args.gotify_token);
        watch_execute(rups_connection, addl_args, notifier).await
    } else {
        list_execute(rups_connection).await
    }
}
