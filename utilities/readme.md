`main.rs` in this directory can be used to make changes to the json test files in the event the schema changes. For e.g. it was used to change goal ids from usize to string. It creates an `input2.json` with whatever changes implemented.

The bash scripts are used after running `main.rs` simply to replace the old input (or output) .json files with the new ones.

To make it possible to run `main.rs`, move it to the `src/` and then it can be run with `cargo run`.