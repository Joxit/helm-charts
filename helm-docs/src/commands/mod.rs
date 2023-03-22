use crate::commands::readme::Readme;
use structopt::StructOpt;

mod readme;
#[derive(Debug, StructOpt)]
pub enum Command {
  /// Build a WOF database (sqlite or shapefile).
  #[structopt(name = "readme")]
  Readme(Readme),
}

impl Command {
  pub fn exec(&self) {
    match self {
      Command::Readme(executable) => executable.exec(),
    }
  }
}
