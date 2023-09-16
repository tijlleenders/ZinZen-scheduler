## Profiling setup

### A) Flamegraph
[Flamegraph](https://github.com/flamegraph-rs/flamegraph) is a profiling tool for rust. The binary defined in [flamegraph-bin](../../src/bin/flamegraph-bin.rs)
defines the tests to run. Flamegraph can be invoked by running 
```shell
cargo flamegraph --bin flamegraph
```
(!) note: this does not currently work on MacOS (see the section `Running flamegraph` in [Troubleshooting](Troubleshooting.md))
