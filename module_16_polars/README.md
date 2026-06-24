# Polars Demo

## Instructions

First, generate a couple of albums datasets.

```bash
python3 albums.py 100 > albums100.csv
python3 albums.py 50000000 > albums50M.csv
```

Then, run the three versions of the code. You can change the file name if you want
to use a smaller or bigger dataset:
```bash
cargo run --release --bin eager -- albums50M.csv
cargo run --release --bin eager_optimized -- albums50M.csv
cargo run --release --bin lazy -- albums50M.csv
```

The lazy file also prints the plan for how polars will execute the query before and after its automatic optimizations.

Look at the time it takes to execute the query in each three cases. Try to reason about why the time is different between them. Try it for a smaller dataset and look at how the time behaves!

Finally, if you want to look at the **peak memory footprint** for each program, you can use either `/usr/bin/time -l` on mac or `/user/bin/time -v` on linux followed by the command to run the program. For example:
```bash
/usr/bin/time -l cargo run --release --bin eager -- albums50M.csv
```