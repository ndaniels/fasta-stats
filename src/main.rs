use bio::io::fasta::Reader;
// use std::collections::HashMap;
use num_format::{Locale, ToFormattedString};
use statistical::{mean, median, standard_deviation};
use std::io;

fn main() {
    let reader = Reader::new(io::stdin());
    // let mut counter: HashMap<usize, usize> = HashMap::new();
    let mut counter = Vec::with_capacity(100_000_000);
    for record in reader.records() {
        let record = record.unwrap();
        let l = record.seq().len();
        counter.push(l as f64);
    }
    let count = counter.len();
    let avg = mean(&counter);
    let med = median(&counter);
    let stddev = standard_deviation(&counter, None);

    println!(
        "Count: {}\nMean: {:.2}\nMedian: {:.2}\nStddev: {:.2}",
        count.to_formatted_string(&Locale::en),
        avg,
        med,
        stddev
    )
}
