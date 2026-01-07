extern crate flate2;

mod macros;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::{BufReader, BufRead};
use std::time::Instant;


fn main() {
    // let args_vec: Vec<String> = args().collect();
    // if args_vec.len() != 2 {
    //     eprintln!("usage:`source` `target");
    //     return;
    // }
    // let input_path = &args_vec[1];
    // let output_path = &args_vec[2];
    // ensure_csv!(input_path, "txt");

    let f = File::open("measurements.txt").unwrap();
    let f = BufReader::new(f);
    let mut stats = BTreeMap::<String, (f64, f64, usize, f64)>::new();
    for line in f.lines() {
        let line = line.unwrap();
        let (station, temperature) = line.split_once(";").unwrap();
        let temperature: f64 = temperature.parse().unwrap();
        let stats = stats.entry(station.to_string()).or_insert((f64::MAX,0.,0, f64::MIN)).or_default();

        stats.0 = stats.0.min(temperature);
        stats.1 += temperature;
        stats.2 += 1;
        stats.3 = stats.3.max(temperature);
    }
    print!("{{");
    let mut stats = stats.into_iter().peekable();
    // let start = In stant::now();
    while let Some((station, (min, sum, count, max))) = stats.next() {
        print!("{station} = {min:.1}/{:.1}/ {max:.1}", sum / (count) as f64);
        if stats.peek().is_some() {
            print!(",");
        }
    }
    print!("}}");
    // let output = File::create(output_path).unwrap();
    // let mut encoder = GzEncoder::new(output, Compression::default());

    
    // copy(&mut input, &mut encoder).unwrap();
    // let finished_output = encoder.finish().unwrap();
    // println!("source len {:?}", input.get_ref().metadata().unwrap().len());
    // println!("Target len: {:?}", finished_output.metadata().unwrap().len());
    // println!("Elapsed: {:?}", start.elapsed());
}
