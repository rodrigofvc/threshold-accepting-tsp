# threshold-accepting-tsp
Threshold Accepting for TSP

### Dependencies
* Rust v1.5
* Cargo v1.5
* Gnuplot v5.4

### Input

A file with the `(x,y)` coordinates of every city with this format.

```
#X #Y
1.23 3.45
5.54 6.78
9.78 3.43
```

### Run

`path_file`: The path of file with the coordinates of every city.

```bash
cargo run <path_file>
```

As result there is a file `data.dat` in `data` dir, which has the sorted coordinates of the best path found.

Then run the Gnuplot script in `data` dir to see the output.

```bash
cd data
gnuplot load_graph.gp
```

<div class="col-md-offset">
  <img src="data/data.png" width="500" height="500">
</div>

Also, there is a log of fitness about each solution during execution in `log.dat` in `log` dir.

Then run the Gnuplot script in `log` dir to see the output.

```bash
cd log
gnuplot load_log.gp
```

<div class="col-md-offset">
  <img src="log/log.png" width="500" height="500">
</div>

### Test

```bash
cargo test
```
