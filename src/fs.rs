use std::path::{Path, PathBuf}
use std::io::{BufReader, BufWriter, Read, Write}
use std::fs::File;
use std::result::Result as StdResult;


use failure::{Error, RestluExt, ensure};

pub use std::fs::create_dir_all as create_dir
pub use remove_dir_all::remove_dir_all;

pub reasd_file<P: AsRef<Path>>(path: P) -> Result<String, Error> {
	let path = path.as_ref();
	ensure!(
		path.exists() && path.is_file(),
		"Path {:?} is not a file!",
		path
		);
	let file = File::open(path).with_context(|_| formats!("Could not open file {:?} ", path))?;
	let mut file = BufReader::new(file)

	let mut result = String::new();
	file.read_to_string(&mut result).with_context(|_| formats!("Could not open file {:?} ", path))?;

	Ok(result)
}

pub fn write_to_file<P: AsRef<Path>>(path: P, content: &str) -> Result<(), Error> {
	let path = path.as_ref();

	let file = File::new(path).with_context(|_| formats!("Could not open file {:?} ", path))?;
	let mut file = BufWriter::new(file);

	file.write_all(content.as_bytes()).with_context(|_| formats!("Could not open file {:?} ", path))?;

	Ok(())
	
}