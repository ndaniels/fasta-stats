# fasta-stats
Compute simple descriptive statistics on a FASTA file

## Usage

```
Simple descriptive statistics on FASTA (biological sequence) data

Usage: fasta-stats [OPTIONS] [FILE]

Arguments:
  [FILE]

Options:
  -m, --median
  -d, --stddev
  -s, --sample <SAMPLE>
      --hint <SIZE_HINT>
  -h, --help              Print help
  -V, --version           Print version
```

By default, this uses a streaming approach to compute mean, min, max, and count. Minimal memory should be required.

If the `median` or `stddev` flags are present, more memory will be required as streaming isn't possible. In order to minimize memory usage, the `sample` argument can be specified; it is interpreted as "1 in n", as in, if `--sample 100` is provided, then an expected 1 in 100 samples will be stored in a vector for purposes of these calculations. Larger values of `sample` will result in lower memory usage but less-accurate computations.

This simple program expects to read FASTA data either on STDIN or from a named file, and will output the total number of sequences, as well as the min, max, mean, and optionally median and standard deviation, of the sequence lengths. If you have a compressed FASTA file, you can pipe it through `zcat` or `gunzip` to decompress it on the fly.
