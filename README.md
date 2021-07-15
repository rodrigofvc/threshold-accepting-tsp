# threshold-accepting-tsp
Threshold Accepting for TSP

### Dependencies
* Rust v1.5
* Cargo v1.5
* Gnuplot v5.4

### Build

```bash
cargo build
```

### Input

A seed for pseudo-random numbers.

A file with the `id` of every city with this format.

```
1,2,3,4,5,6,7,75,163,164,165,....
```

### Run

`seed`: The seed for pseudo-random numbers.

`path_file`: The path of file with the id of every city.

```bash
cargo run <seed> <path_file>
```

As result there is a file `data.dat` in `data` dir, which has the sorted coordinates of the best path found.

Then run the Gnuplot script in `data` dir to see the output.

```bash
cd data
gnuplot load_graph.gp
```

<div class="col-md-offset">
  <img src="data/data.png">
</div>

### Example

```bash
cargo run 654 examples/example-1.txt
```

### Test

```bash
cargo test
```
