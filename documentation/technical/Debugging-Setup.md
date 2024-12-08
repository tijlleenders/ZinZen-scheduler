## Debugging setup

### A) Pretty printing date and time

The ZinZen&reg;-scheduler codebase uses the Chrono NaiveDateTime structs for representing
date and time. During debugging these are not printed in a convenient format.

To enable pretty printing of dates we can alter the way the debugger prints these values. This works differently for
each
debugger and platform. A list of confirmed and unconfirmed solutions:

#### A.0) RustRover

- Pin the Rust toolchain to whatever version works with RustRover (in dec '24 I had to go back to 1.81.0),  
  by renaming "stable" to "1.81.0" in `rust-toolchain.toml`.
- Now we still need custom types for chrono.  
  Do this by setting the RustRover Settings=>Build, Execution, Deployment=>Debugger=>Data Views=>Rust LLDB Renderers
  to 'Rust compiler's renderers'.  
  Now you can check the path used for launching LLDB commands in the LLDB tab of the debugger.  
  For me this is '~/.rustup/toolchains/1.81.0-x86_64-unknown-linux-gnu/lib/rustlib/etc/lldb_commands'.
  Append the instructions in 'scripts/debug_formatter/chrono_formatter' to that file, but be careful to adjust the
  chrono.py filepath.  
  I copied the 'scripts/debug_formatter/chrono.py' file to the lldb_commands directory, for easy reference.

#### A.1) Intellij with intellij-rust plugin

this solution is confirmed to work with Intellij using an LLDB-based debugger on MacOS:
https://github.com/intellij-rust/intellij-rust/issues/10219#issuecomment-1464970768

A similar solution should also work for GDB-based debuggers: https://github.com/intellij-rust/intellij-rust/issues/3639

#### A.2) debugger visualization attributes for rust-native debugging

Rust introduced debugger visualization attributes in
1.71.0: https://doc.rust-lang.org/nightly/reference/attributes/debugger.html#the-debugger_visualizer-attribute. This
should also work, but is not yet tested in the context of this codebase.

#### A.3) VSCode

Install the required extensions for debugging Rust.

- C/C++ for VSCode if you're running Windows
- CodeLLDB if you're running Linux

To debug tests with better `NaiveDateTime` formatter, kindly add custom configuration inside `launch.json` as below:

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug 'rust_tests' with pprint",
      "cargo": {
        "args": [
          "test",
          "--no-run",
          "--test=rust_tests",
          "--package=zinzen"
        ],
        "filter": {
          "name": "rust_tests",
          "kind": "test"
        }
      },
      "args": [],
      "cwd": "${workspaceFolder}",
      "initCommands": [
        "command source '${workspaceFolder}/scripts/debug_formatter/chrono_formatter'"
      ]
    }
  ]
}
```

If you get an error `no rust_tests found` do a `cargo build` and try again.

To debug a single, specific test, from the rust_tests:  
add the test name to the root args (not the cargo args) as such:

```
"args": ["test_name_here"], 
```