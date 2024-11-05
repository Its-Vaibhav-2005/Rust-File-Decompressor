extern crate flate2;

use flate2::read::GzDecoder;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn main(){
    if args().len() != 3{
        eprintln!("Usage: `source` `target`");
        return;
    }
    let input = BufReader::new(
        File::open(args().nth(1).unwrap()).unwrap()
    );
    let mut out = File::create(
        args().nth(2).unwrap()
    ).unwrap();
    let mut decoder = GzDecoder::new(
        input
    );
    let start = Instant::now();
    copy(
        &mut  decoder,
        &mut out
    ).unwrap();
    println!(
        "Decompressed length: {:?}",
        out.metadata().unwrap().len()
    );
    println!(
        "Elapsed: {:?}",
        start.elapsed()
    );
}