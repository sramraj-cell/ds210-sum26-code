use std::env;
use std::time::Instant;
use polars::prelude::*;

fn main() {
    let timer = Instant::now();

    // Read dataset filename from the command line arguments.
    let args: Vec<String> = env::args().collect();
    let filename: &str = &args[1];

    // Configure a lazy read for the csv file: this does not read the data yet!
    let data = LazyCsvReader::new(filename.into())
        .with_has_header(true)
        .finish().unwrap();

    // Write down the query. It does not have to be in optimized form!
    let output = data
        .group_by([col("band"), col("album")])
        .agg([col("rating").mean()])
        .filter(col("album").eq(lit("Ashen")));

    // Print the unoptimized query plan
    println!("Initial Plan: {}", output.explain(false).unwrap());

    // Print the automatically ***optimized*** query plan
    println!("Optimized Plan: {}", output.explain(true).unwrap());

    // Ask polars to run the query (with optimizations).
    let result = output.collect().unwrap();

    // Print results.
    println!("{}", result);
    println!("{:?}", timer.elapsed());
}
