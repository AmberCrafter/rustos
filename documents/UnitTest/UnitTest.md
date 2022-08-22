# Unit Test

ref: https://os.phil-opp.com/testing/

## Work flow
1. Set custom_test_framework
```rust
#![feature(custom_test_framework)]
```

2. Set test_runner
```rust
fn test_runner(tests: &[&dyn Fn()]) {
    unimplemented!();
}
```

3. Execute test in main/start/entry function
```rust
#![reexport_test_harness_main = "test_main"]
...

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}
```