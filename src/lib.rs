pub mod minigrep;

use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use crate::minigrep::config;

pub fn run(config: config::Config) -> Result<(), Box<Error>> {
    let f = File::open(config.filename)?;
    let reader = BufReader::new(f);
    minigrep::search(&config.query, reader.lines(), |line| println!("{}", line));
    Ok(())
}
