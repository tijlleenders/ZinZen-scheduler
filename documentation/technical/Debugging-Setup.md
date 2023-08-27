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
There is an existing workaround for VSCode - not yet tested in context of this codebase: https://stackoverflow.com/questions/68553738/how-do-i-see-a-user-friendly-format-when-debugging-chronodatetime-in-vscode-ll