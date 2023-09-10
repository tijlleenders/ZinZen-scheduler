## Troubleshooting

Random errors that are encountered, along with their respective fixes. 
This should be the first place to look when you encounter any difficulties.

Feel free to add to this when encountering any troubles.

### Running deno_test script

#### scheduler.wasm: no such file or directory
* issue: `Bindgen...cp: target/wasm32-unknown-unknown/release/scheduler.wasm: No such file or directory`
on running deno_test.sh

* solution: make sure you run the script with your current working dir in root.

#### Bindgen..../scripts/deno_test.sh: line 18: wasm-bindgen: command not found
* issue: wasm-bindgen is not installed but called from deno_test.sh  

* solution: install wasm-bindgen using `cargo install -f wasm-bindgen-cli`
(make sure openssl and pkg-config are installed on your system first)

#### wasm-strip: command not found
* issue: `wasm-strip: command not found` message when running deno_test.sh

* proposed solution: install wasm-strip. Comes with 'wabt': `brew install wabt`. Confirmed to work on MacOS

#### wasm-opt: command not found
* issue: `wasm-opt: command not found`

* proposed solution: install wasm-opt: `cargo install wasm-opt --locked`

