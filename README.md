## README

Reproduce `llvm-cov` with `nextest` flag error.

**Works with nextest**
``` sh
[nix-shell:~/projects/llvm-cov-err]$ cargo nextest run -E "not package(exclude)"
   Compiling exclude v0.1.0 (/home/noel/projects/llvm-cov-err/exclude)
   Compiling include v0.1.0 (/home/noel/projects/llvm-cov-err/include)
    Finished test [unoptimized + debuginfo] target(s) in 0.87s
    Starting 1 tests across 2 binaries
        PASS [   0.001s] include::bin/include test::include_test
------------
     Summary [   0.002s] 1 tests run: 1 passed, 0 skipped
```
     
**Fails with llvm-cov wrapper**
```sh
[nix-shell:~/projects/llvm-cov-err]$ cargo llvm-cov nextest run -E "not package(exclude)"
error: error: Found argument '-E' which wasn't expected, or isn't valid in this context

        If you tried to supply `-E` as a value rather than a flag, use `-- -E`

USAGE:
    cargo llvm-cov run [OPTIONS] [-- <ARGS>...]

For more information try --help

```

**Fails with llvm-cov wrapper using workspace and exclude flags**
```sh
[nix-shell:~/projects/llvm-cov-err]$ cargo llvm-cov nextest --workspace --exclude "not package(exclude)"
   Compiling include v0.1.0 (/home/noel/projects/llvm-cov-err/include)
   Compiling exclude v0.1.0 (/home/noel/projects/llvm-cov-err/exclude)
    Finished test [unoptimized + debuginfo] target(s) in 0.25s
    Starting 2 tests across 2 binaries
        FAIL [   0.003s] exclude::bin/exclude test::exclude_test

--- STDOUT:              exclude::bin/exclude test::exclude_test ---

running 1 test
test test::exclude_test ... FAILED

failures:

failures:
    test::exclude_test

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


--- STDERR:              exclude::bin/exclude test::exclude_test ---
thread 'main' panicked at 'this should be excluded', exclude/src/main.rs:9:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

   Canceling due to test failure: 1 tests still running
        PASS [   0.003s] include::bin/include test::include_test
------------
     Summary [   0.005s] 2 tests run: 1 passed, 1 failed, 0 skipped
error: test run failed
error: process didn't exit successfully: `/home/noel/.rustup/toolchains/nightly-2022-06-20-x86_64-unknown-linux-gnu/bin/cargo nextest run --tests --manifest-path /home/noel/projects/llvm-cov-err/Cargo.toml --target-dir /home/noel/projects/llvm-cov-err/target/llvm-cov-target` (exit status: 100)
```
