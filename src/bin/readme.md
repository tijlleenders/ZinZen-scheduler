An executable is needed in order to profile the code using flamegraph. `main.rs` is the executable that is used for this purpose. It runs the scheduler against the inputs in tests/jsons.

To generate a flamegraph of the scheduler, run ``.

`modify.rs` is more of a utility. It is used to make time-consuming modifications to the json test files in the event the schema changes. For e.g. it was used when we needed to change goal ids from usize to string. It creates either a `input2.json` or `output2.json` in each sub-directory, with the new versions of the respective files. Then you can use the bash scripts in /utilities to replace the old input (or output) .json files with the new ones.

To make changes to the json test files, first set up `modify.rs` as needed, then do `cargo run --bin modify`.