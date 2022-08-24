# Basic Usage

### File structure
* {crate-name}/
    + .cargo/
        - config.toml : cargo command script
    + image_maker/
        - src/
            - main.rs : building process and Qemu executing args (modify if need)
    + src/ : source code
        - library/
        - lib.rs
        - main.rs
    + target/ : cargo build results
        - x86_64-rustos-none/ : Qemu execute target folder
            - debug/
                - deps/
                    - boot-bios-interrupt-b5aa4371b9e2d1be.img : Qemu execute target with `cargo ktestfile interrupt`
                    - interrupt-b5aa4371b9e2d1be : symbol file for `boot-bios-interrupt-b5aa4371b9e2d1be.img`
                    - ...
                - boot-bios-rustos.img : Qemu execute target with `cago krun`
                - rustos : symbol file for `boot-bios-rustos.img`
                - ...
            - ...
        - ...
    + tests/
        - interrupt.rs
        - ...
    + x86_64-rustos-none.json : [target Specification file](https://os.phil-opp.com/minimal-rust-kernel/#target-specification) 

> note. all the files and folders name of `x86_64-rustos-none` are depended on the target file (***.json)

---
Step:
1. Modified Qemu executing arguments
```rust
// image_marker/src/main.rs

// const RUN_ARGS: &[&str] = &["--no-reboot", "-s", "-nographic", "-monitor", "telnet::45454,server,nowait", "-serial", "mon:stdio"];
// const RUN_ARGS: &[&str] = &["--no-reboot", "-s", "-nographic", "-serial", "mon:stdio"];
// const RUN_ARGS: &[&str] = &["--no-reboot", "-s", "-serial", "mon:stdio"];
// const RUN_ARGS: &[&str] = &["--no-reboot", "-s"];

// This use on non-UI environment
// const RUN_ARGS: &[&str] = &["--no-reboot", "-s", "-display", "none", "-serial", "mon:stdio"];
// const RUN_ARGS: &[&str] = &["-d", "int", "--no-reboot", "-s", "-display", "none", "-serial", "mon:stdio"];
const RUN_ARGS: &[&str] = &["-d", "int", "--no-reboot", "-s", "-S", "-display", "none", "-serial", "mon:stdio"];
...

```

Descriptions:
 - `-d int`: dump the cup information, just likes `info registors` for gdb
 - `--no-reboot`: without reboot after system crash, ie. run into triple fault
 - `-s`: stop the programe at the start position when Qemu executed
 - `-S`: waiting for gdb connection
 - `-disply none`: without gui, it's useful to run the emulation on the remote server without graphic environments, ie. x-window
 - `-serial mon:stdio`: redirection serial port to current console stdin/stdout

> note. ctrl+c not work under `-serial mon:stdio` enable, need to use `crtl a + x` (two step command) to close the Qemu. (`crtl a + h` show more command)


2. Execute Qemu & GDB connect

 * main program case
 ```bash
 # console 1
 ## Qemu will execute the `target/x86_64-rustos-none/debug/boot-bios-rustos.img
 > cargo krun

 # console 2
 > rust-gdb
 # select symbol file
 (gdb) file target/x86_64-rustos-none/debug/rustos
 (gdb) target remote :1234
 # set the brackpoint at src/main.rs n line
 (gdb) b src/main.rs: n
 # continue run, stop at the brackpoint `src/main.rs: n` in this case
 (gdb) c
 ...

 ```


  * test program case
 ```bash
 # console 1
 ## specified tests/interrupt.rs to run testing
 ## cargo ktestfile is defined at `./cargo/config.toml`
 ## Qemu will execute the `target/x86_64-rustos-none/debug/deps/boot-bios-interrupt-b5aa4371b9e2d1be.img`
 > cargo ktestfile interrupt

 # console 2
 > rust-gdb
 # select symbol file
 (gdb) file target/x86_64-rustos-none/debug/deps/interrupt-b5aa4371b9e2d1be
 (gdb) target remote :1234
 # set the brackpoint at tests/interrupt.rs n line
 (gdb) b tests/interrupt.rs: n
 # continue run, stop at the brackpoint `tests/interrupt.rs: n` in this case
 (gdb) c
 ...
 
 ```