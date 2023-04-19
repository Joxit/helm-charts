use crate::readme::generate_readme;
use anyhow::Result;
use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Readme {
  #[structopt(short = "w", long = "write")]
  write: bool,
  #[structopt(required = true)]
  directories: Vec<PathBuf>,
}

impl Readme {
  pub fn exec(&self) -> Result<()> {
    for directory in &self.directories {
      if self.write {
        let mut file = File::create(directory.join("README.md"))?;
        generate_readme(&mut file, &directory)?;
      } else {
        generate_readme(&mut std::io::stdout(), &directory)?;
      }
    }
    Ok(())
  }
}
