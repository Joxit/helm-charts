use crate::commands::Command;
use structopt::StructOpt;

mod commands;
mod table;
mod usage;

#[derive(Debug, StructOpt)]
#[structopt(name = "helm-docs", author, about)]
pub struct HelmDocs {
  #[structopt(subcommand)]
  pub command: Command,
}

fn main() {
  let opt = HelmDocs::from_args();

  opt.command.exec();
}
