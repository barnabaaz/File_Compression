use flate2::{write::GzEncoder, Compression};
use std::error::Error;
use std::fs::File;
use std::io::{copy, BufReader};
use std::process;
use std::time::Instant;
pub struct Config {
    filename: String,
    compressed_filename: String,
}
impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();
        let filename = match args.next() {
            Some(args) => args,
            None => return Err("didn't get source a file name"),
        };
        let compressed_filename = match args.next() {
            Some(args) => format!("{}.gz", args),
            None => return Err("didn't get target file name"),
        };
        Ok(Config {
            filename,
            compressed_filename,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut input = BufReader::new(File::open(config.filename).unwrap_or_else(|err| {
        eprintln!(
            "[!] An error occured when trying to access the file : {}",
            err
        );
        process::exit(1)
    }));
    let output = File::create(config.compressed_filename).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());
    let time_taken = Instant::now();
    copy(&mut input, &mut encoder).unwrap_or_else(|err| {
        eprintln!("An error occured during compression: {}", err);
        process::exit(1)
    });
    let output = encoder.finish().unwrap();
    println!(
        "Source Length {:?}",
        input.get_ref().metadata().unwrap().len()
    );
    println!("target Length {:?}", output.metadata().unwrap().len());
    println!("Time Elasped for enconding {:?}", time_taken.elapsed());
    Ok(())
}
