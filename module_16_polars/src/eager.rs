use std::env;
use std::time::Instant;
use polars::prelude::*;

fn main() {
    let timer = Instant::now();

    // Read dataset filename from the command line arguments.
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // Set up a reader object to read the csv file (but do not read yet).
    let reader = 
        CsvReadOptions::default()
            .with_has_header(true)
            .try_into_reader_with_file_path(Some(filename.into()))
            .unwrap();

    // Read the entirety of the file into the `data` variable.
    let data = reader.finish().unwrap();
    println!("{}", data);
    println!("Number of rows in dataset {}", data.height());
    println!("First row in dataset {:?}", data.get_row(0).unwrap());


    // Group by the band and album columns, then compute the average rating per
    // group.
    let groups =
        data.group_by(["band", "album"]).unwrap();
    let averages =
        groups.select(["rating"]).mean().unwrap();

    // Filter by album == Ashen.
    let condition =
        averages.column("album").unwrap().str().unwrap()
        .equal("Ashen");
    let result = averages.filter(&condition).unwrap();

    // Print result.
    println!("{}", result);
    println!("{:?}", timer.elapsed());
}
