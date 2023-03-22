use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;
mod commands;
mod table;
use crate::commands::Command;
use crate::table::generate_table;

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
