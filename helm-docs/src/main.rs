use crate::commands::Command;
use structopt::StructOpt;

mod chart;
mod commands;
mod readme;

#[derive(Debug, StructOpt)]
#[structopt(name = "helm-docs", author, about)]
pub struct HelmDocs {
  #[structopt(subcommand)]
  pub command: Command,
}

fn main() {
  let opt = HelmDocs::from_args();

  if let Err(e) = opt.command.exec() {
    eprintln!("Error: {}", e);
    let root_cause = format!("{}", e.root_cause());
    if !root_cause.is_empty() {
      eprintln!("");
      eprintln!("Caused by:");
      eprintln!("    {}", e.root_cause());
    }
  }
}
