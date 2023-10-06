extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn main() {
    //checking for cmd_l args to >= 3
    if args().len() != 3 {
        eprintln!("Usage: `source` `target`");
        return;
    }
    //open file specified as the 2nd arg (and wrap)
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    //create new file as (3rd arg)
    let output = File::create(args().nth(2).unwrap()).unwrap();
    //compressing>>>
    let mut encoder = GzEncoder::new(output, Compression::default());

    let start = Instant::now();

    let duration = start.duration_since(start);

    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    println!(
        "Source len: {:?}",
        input.get_ref().metadata().unwrap().len()
    );
    println!("Target len: {:?}", output.metadata().unwrap().len());
    println!("Elapsed: {:?}", start.elapsed());
    println!(
        "Spent: {:?} milliseconds compressing this file",
        duration.as_millis()
    );
}
