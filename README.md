# A simple template for writing quick `WebAssembly` binaries in Rust

Simply edit `src/libs.rs`, then run `./build.sh`. The output file: `out/output.wasm`, is the resulting `WebAssembly`.

It only requires that you give it an `exit` function: **`fn exit(error_type: u8) -> !`**.

To run `./build.sh` without errors, ensure you have `wabt`  and `wasm-opt`installed

## Usage

```javascript
const fs = require("fs");
const assert = require("assert");

WebAssembly.instantiate(
   fs.readFileSync("out/output.wasm"),
   // Bare-bones import object
   {
      "env": {
         "exit": (error_type) => {
            switch (error_type) {
               case 0:
                  console.log("Normal exit")
                  break;
               case 1:
                  console.log("Some error occurred");
                  break;
               default:
                  console.error("Unknown error occurred");
                  break;
            }
         },
      },
   }
).then(({ instance }) => {
   assert(instance.exports.add(1, 2) == 3)
});
```
