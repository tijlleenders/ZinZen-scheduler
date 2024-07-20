## Development setup
1. Clone repo
   ```shell
   # Go to Home Directory
   cd ~ 

   # You can clone into home directory or different directory
   git clone https://github.com/tijlleenders/ZinZen-scheduler.git
   ```  

2. [Install Rust](https://www.rust-lang.org/tools/install)

3. Add target for wasm:

   ```shell
   rustup target add wasm32-unknown-unknown  

   # Go to project directory
   cd ~/ZinZen-scheduler/
   ```

4. Install WASM dependencies
   ```shell
   cargo install wasm-bindgen-cli 
   ```
    - Or [Install wasm-bindgen command line interface](https://rustwasm.github.io/wasm-bindgen/reference/cli.html) with any dependencies
    - If it fails with openssl or pkg-config error message do
   ```shell
   sudo apt-get install wabt binaryen
   # [... or DIY](https://github.com/WebAssembly/wabt)
   ```
   - If it fails with `linker 'cc' not found` do `sudo apt install build-essential` and retry

5. (Optional) [Install deno](https://deno.land/manual/getting_started/installation). Only necessary if you want to run the deno tests that test the usage of the WASM module in JavaScript context.

6. You can now run the test from javascript/deno or from `cargo` as below:
   ```shell
   # Run tests by deno 
   ./deno_test.sh

   # Run tests by cargo
   cargo test
   ```

7. Debugging: extra, useful information for an efficient debugging setup can be found at [Debugging-Setup](Debugging-Setup.md)
8. Profiling: methods for profiling are described in [Profiling](Profiling.md)