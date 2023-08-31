## Debugging setup

### A) Pretty printing date and time

The ZinZen&reg;-scheduler codebase uses the Chrono NaiveDateTime structs for representing
date and time. During debugging these are not printed in a convenient format.

To enable pretty printing of dates we can alter the way the debugger prints these values. This works differently for each 
debugger and platform. A list of confirmed and unconfirmed solutions:

#### A.1) Intellij with intellij-rust plugin
this solution is confirmed to work with Intellij using an LLDB-based debugger on MacOS:
https://github.com/intellij-rust/intellij-rust/issues/10219#issuecomment-1464970768

A similar solution should also work for GDB-based debuggers: https://github.com/intellij-rust/intellij-rust/issues/3639

#### A.2) debugger visualization attributes for rust-native debugging
Rust introduced debugger visualization attributes in 1.71.0: https://doc.rust-lang.org/nightly/reference/attributes/debugger.html#the-debugger_visualizer-attribute. This should also work, but is not yet tested in the context of this codebase.

#### A.3) VSCode
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