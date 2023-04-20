use crate::readme;
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
      let readme = readme::Readme::new(&directory)?;
      if self.write {
        let mut file = File::create(directory.join("README.md"))?;
        readme.write(&mut file)?;
      } else {
        readme.write(&mut std::io::stdout())?;
      }
    }
    Ok(())
  }
}
