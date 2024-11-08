use std::fs;
use std::io;
use std::path::Path;

pub fn rename_file<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> io::Result<()> {
  fs::create_dir_all(to.as_ref().parent().unwrap())?;
  // Use copy instead of rename. To avoid invalid cross-device link error of docker
  fs::copy(from.as_ref(), to.as_ref())?;
  fs::remove_file(from.as_ref())?;

  Ok(())
}
