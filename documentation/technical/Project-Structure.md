## Project Structure

### Entrypoints
src/lib.rs contains the 2 main entrypoints of the code:

1) run-scheduler 
   1) this is the main entry point for calling the scheduling algorithm as a Rust program
2) schedule
   1) this is the entry point for the exposed WASM module. should do the same as run-scheduler, without the logging.

### Tests
Functions, and implementations are tested with standard unit tests.

The folder root/tests/jsons contains expected in- and corresponding outputs for 
end-to-end test scenarios. The code to run them is auto-generated at compile time.

We can also run the tests in a javascript environment as WASM=module using Deno as javascript runtime.
The entrypoint for these tests is tests/deno_tests.ts, and it can be run by executing ./scripts/deno_tests.sh.
This is meant as an integration test to see everything runs fine in a javascript context.
