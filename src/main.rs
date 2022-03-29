use tracing_subscriber::EnvFilter;
use color_eyre::Report;

mod notify;
use notify::*;

mod ups_state;
pub use ups_state::*;

mod args;
use args::Top;

mod commands;
use commands::{Command, Watch, List};

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
        args::SubCommand::ListVars(args) => {
            let blank_notify = GotifyNotifier::new("".to_string(), "".to_string());
            List::execute(args, blank_notify)
        },
        args::SubCommand::Watch(args) => {
            let notifier = GotifyNotifier::new(args.gotify_url.clone(), args.gotify_token.clone());
            Watch::execute(args, notifier)
        },
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn smoke() {
        
    }
}