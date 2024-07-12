use bio::io::fasta::Reader;
use clap::Parser;
use num_format::{Locale, ToFormattedString};
use rand::Rng;
use std::fs::File;
use std::io;

mod stats;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]

struct Args {
    // Median
    #[clap(short = 'm', long = "median")]
    median: bool,
    // Standard Deviation
    #[clap(short = 'd', long = "stddev")]
    stddev: bool,
    // Sampling
    #[clap(short = 's', long = "sample")]
    sample: Option<u32>,
    #[clap(long = "hint")]
    size_hint: Option<usize>,
    // Filename (or stdin)
    file: Option<String>,
}

fn main() {
    let args = Args::parse();
    let input: Box<dyn io::Read> = match args.file {
        Some(filename) => Box::new(File::open(filename).unwrap()),
        None => Box::new(io::stdin()),
    };
    let reader = Reader::new(input);
    let size_hint = args.size_hint.unwrap_or(100_000_000);
    let sample_rate = args.sample.unwrap_or(1);
    let mut sampler = if args.median || args.stddev {
        Vec::with_capacity(size_hint)
    } else {
        vec![]
    };
    let mut count = 0_usize;

    let mut rng = rand::thread_rng();
    let mut sum = 0_usize;
    let mut uninit = true;
    let mut min = 0;
    let mut max = 0;
    for record in reader.records() {
        let record = record.unwrap();
        let l = record.seq().len();
        let r = rng.gen_range(0..sample_rate);
        if (args.median || args.stddev) && r <= 1 {
            sampler.push(l);
        }
        count += 1;
        sum += l;
        if uninit {
            min = l;
            max = l;
            uninit = false;
        } else {
            if l < min {
                min = l;
            }
            if l > max {
                max = l;
            }
        }
    }
    let avg = sum as f64 / count as f64;

    let med = if args.median {
        stats::median(&sampler).unwrap()
    } else {
        0
    };

    let stddev = if args.stddev {
        stats::par_stddev(&sampler, Some(avg))
    } else {
        0.0
    };

    eprintln!("Sampler length: {}", sampler.len());
    println!(
        "Count: {}\nMin:{}\nMax:{}\nMean: {:.2}",
        count.to_formatted_string(&Locale::en),
        min,
        max,
        avg
    );
    if args.median {
        println!("Median: {med}");
    };
    if args.stddev {
        println!("Stddev: {:.2}", stddev);
    };
}
