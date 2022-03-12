use argh::FromArgs;

#[derive(FromArgs, Debug)]
#[argh(description = "Commands")]
pub struct Top {
  #[argh(subcommand)]
  pub command: SubCommand,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
pub enum SubCommand {
    Watch(NotifyArgs)
}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "notify", description = "Notify on NUT state changes")]
pub struct NotifyArgs {
    #[argh(option,  description = "push url", short = 'u')]
    pub url: String,
    #[argh(option, description = "poll duration", short = 'p')]
    pub p: usize,
}