# fasta-stats
Compute simple descriptive statistics on a FASTA file

## Usage
```fasta-stats < pdb_seqres.fasta```

This simple program expects to read FASTA data on STDIN and will output the total number of sequences, as well as the mean, median, and standard deviation of the sequence lengths. If you have a compressed FASTA file, you can pipe it through `zcat` or `gunzip` to decompress it on the fly.
