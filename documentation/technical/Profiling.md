## Profiling setup

### Samply

Samply is a general-purpose sampler that uses the firefox profiler.
https://github.com/mstange/samply/

### DHAT

DHAT is a heap allocation profiler.  
You can launch the profiler using `cargo t --release --features dhat-heap --test rust_tests e2e::tijl_goals`.  
Launch all tests in single-threaded mode - since there can't be two profilers running at the same
time: `cargo t --release --features dhat-heap --test rust_tests -- --test-threads=1
`.  
View the `dhat-heap.json` with an on/offline viewer, for
example https://nnethercote.github.io/dh_view/dh_view.html