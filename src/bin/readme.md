There are two executables in this directory.

1) An executable is needed in order to profile the code using flamegraph. `main.rs` is the executable used for this. It runs the scheduler against the inputs in `tests/jsons`.

To generate a flamegraph of the scheduler on your machine, follow the platform-specific instructions [here](https://github.com/flamegraph-rs/flamegraph). 

2) `modify.rs` is more of a utility I use to make time-consuming modifications to the json test files such as when the schema changes. For e.g. it was used when we needed to change goal ids from usize to string. To make changes to the json test files, first set up `modify.rs` as needed, then do `cargo run --bin modify`. It creates either a `input2.json` or `output2.json` in each sub-directory in `tests/jsons`, with the new versions of the respective files. Then the bash scripts in `/utilities` can be used to replace the old input (or output) .json files with the new ones. 