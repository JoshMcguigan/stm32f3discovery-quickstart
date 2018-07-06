# stm32f3discovery-quickstart

This repository houses a Bash script and a few auxiliary files which can initialize a new Rust project targeting the STM32F3DISCOVERY development board in a single command.

```
$ git clone https://github.com/JoshMcguigan/stm32f3discovery-quickstart.git
    Cloning into 'stm32f3discovery-quickstart'...
$ ./stm32f3discovery-quickstart/init
    Project Name: newest-micro-project
        Created binary (application) `newest-micro-project` project
$ cd newest-micro-project/
$ cargo run
    Updating registry `https://github.com/rust-lang/crates.io-index`
    Compiling cc v1.0.17                                                         
    ...
    Compiling newest-micro-project v0.1.0 (file:///Users/josh/Projects/newest-micro-project)
     Finished dev [unoptimized + debuginfo] target(s) in 41.82s
      Running `arm-none-eabi-gdb target/thumbv7em-none-eabihf/debug/newest-micro-project`
```

The `init` script asks only for a project name, and then it generates a new Cargo project and sets everything up to compile for the STM32F3DISCOVERY.

For a more details, check out this [introductory blog post](https://www.joshmcguigan.com/blog/stm32f3discovery-quickstart/).

## Requirements

If you are new to embedded development in Rust, start by [setting up a development environment](https://japaric.github.io/discovery/03-setup/README.html).
