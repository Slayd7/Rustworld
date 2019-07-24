use ggez::*;
use crate::rw_tile_lib::Lookup;
use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;

fn visit_dirs(dir: &Path, cb: &Fn(&DirEntry)) -> io::Result<()> {
  if dir.is_dir() {
    for entry in fs::read_dir(dir)? {
      let entry = entry?;
      let path = entry.path();
      println!("{}", path.display());

    }
  }
  Ok(())

}

pub fn import_tiles(mut lib: &Lookup) -> ()
{
  
}

