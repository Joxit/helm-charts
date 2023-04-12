use crate::commands::check::Check;
use crate::commands::readme::Readme;
use anyhow::Result;
use structopt::StructOpt;

mod check;
mod readme;

#[derive(Debug, StructOpt)]
pub enum Command {
  /// Generate README.md for charts.
  #[structopt(name = "readme")]
  Readme(Readme),
  /// Check if the versions in the chart are consistent.
  #[structopt(name = "check")]
  Check(Check),
}

impl Command {
  pub fn exec(&self) -> Result<()> {
    match self {
      Command::Readme(executable) => executable.exec()?,
      Command::Check(executable) => executable.exec()?,
    }
    Ok(())
  }
}
