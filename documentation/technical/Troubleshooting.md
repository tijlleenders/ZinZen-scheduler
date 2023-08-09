## Troubleshooting

Random errors that are encountered, along with their respective fixes. 
This should be the first place to look when you encounter any difficulties.

Feel free to add to this when encountering any troubles.

### Running deno_test_script

#### scheduler.wasm: no such file or directory
* issue: `Bindgen...cp: target/wasm32-unknown-unknown/release/scheduler.wasm: No such file or directory`
on running deno_test_script.sh

* solution: make sure you run the script with your current working dir in root.

#### wasm-strip: command not found
* issue: `wasm-strip: command not found` message when running deno_test_script.sh

* proposed solution: install wasm-strip. Comes with 'wabt': `brew install wabt`. Confirmed to work on MacOS

#### wasm-opt: command not found
* issue: `wasm-opt: command not found`

* proposed solution: install wasm-opt: `cargo install wasm-opt --locked`

