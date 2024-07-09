use rayon::prelude::*;
use std::cmp::Ordering;

pub fn par_mean(v: &[usize]) -> f64 {
    let l = v.len() as f64;
    let sum: usize = v.par_iter().sum();
    sum as f64 / l
}

pub fn par_min(v: &[usize]) -> usize {
    *v.par_iter().min().unwrap()
}

pub fn par_max(v: &[usize]) -> usize {
    *v.par_iter().max().unwrap()
}

pub fn par_stddev(v: &[usize], avg: Option<f64>) -> f64 {
    let mean = match avg {
        Some(val) => val,
        None => par_mean(v),
    };
    let count = v.len();
    let variance = v
        .par_iter()
        .map(|value| {
            let diff = mean - (*value as f64);

            diff * diff
        })
        .sum::<f64>()
        / count as f64;
    variance.sqrt()
}

// The below is adapted from the Rust Cookbook
fn partition(data: &[usize]) -> Option<(Vec<usize>, usize, Vec<usize>)> {
    match data.len() {
        0 => None,
        _ => {
            let (pivot_slice, tail) = data.split_at(1);
            let pivot = pivot_slice[0];
            let (left, right) = tail.iter().fold((vec![], vec![]), |mut splits, next| {
                {
                    let (ref mut left, ref mut right) = &mut splits;
                    if next < &pivot {
                        left.push(*next);
                    } else {
                        right.push(*next);
                    }
                }
                splits
            });

            Some((left, pivot, right))
        }
    }
}

fn select(data: &[usize], k: usize) -> Option<usize> {
    let part = partition(data);

    match part {
        None => None,
        Some((left, pivot, right)) => {
            let pivot_idx = left.len();

            match pivot_idx.cmp(&k) {
                Ordering::Equal => Some(pivot),
                Ordering::Greater => select(&left, k),
                Ordering::Less => select(&right, k - (pivot_idx + 1)),
            }
        }
    }
}

pub fn median(data: &[usize]) -> Option<usize> {
    let size = data.len();

    match size {
        even if even % 2 == 0 => {
            let fst_med = select(data, (even / 2) - 1);
            let snd_med = select(data, even / 2);

            match (fst_med, snd_med) {
                (Some(fst), Some(snd)) => Some(fst),
                _ => None,
            }
        }
        odd => select(data, odd / 2),
    }
}
