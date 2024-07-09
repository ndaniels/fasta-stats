use bio::io::fasta::Reader;
mod stats;
use num_format::{Locale, ToFormattedString};
use std::io;

fn main() {
    let reader = Reader::new(io::stdin());
    let mut counter = Vec::with_capacity(100_000_000);
    for record in reader.records() {
        let record = record.unwrap();
        let l = record.seq().len();
        counter.push(l);
    }
    let count = counter.len();
    let min = stats::par_min(&counter);
    let max = stats::par_max(&counter);
    let avg = stats::par_mean(&counter);
    let med = stats::median(&counter).unwrap();
    let stddev = stats::par_stddev(&counter, Some(avg));

    println!(
        "Count: {}\nMin:{}\nMax:{}\nMean: {:.2}\nMedian: {}\nStddev: {:.2}",
        count.to_formatted_string(&Locale::en),
        min,
        max,
        avg,
        med,
        stddev
    )
}
