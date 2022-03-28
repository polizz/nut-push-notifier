use tracing_subscriber::EnvFilter;
use color_eyre::Report;

mod notify;
mod ups_state;
pub use ups_state::*;

mod args;
use args::Top;

mod commands;
use commands::{Command, Watch, list};

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
        args::SubCommand::ListVars(args) => list(args),
        args::SubCommand::Watch(args) => Watch::execute(args),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn smoke() {
        
    }
}