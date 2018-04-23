# Robust Arduino Serial Protocol in Rust

[![Build Status](https://travis-ci.org/araffin/rust-arduino-serial.svg?branch=master)](https://travis-ci.org/araffin/rust-arduino-serial) [![Build status](https://ci.appveyor.com/api/projects/status/h0ejgesat0nnpahc/branch/master?svg=true)](https://ci.appveyor.com/project/araffin/rust-arduino-serial/branch/master) [![Crates.io](https://img.shields.io/badge/crates.io-v0.1.0-orange.svg?longCache=true)](https://crates.io/crates/robust-arduino-serial)

`robust_arduino_serial` is a simple and robust serial communication protocol. It was designed to make two arduinos communicate, but can also be useful when you want a computer (e.g. a Raspberry Pi) to communicate with an Arduino.

This repository is part of the Robust Arduino Serial project, main repository: [https://github.com/araffin/arduino-robust-serial](https://github.com/araffin/arduino-robust-serial)

**Please read the [Medium Article](https://medium.com/@araffin/simple-and-robust-computer-arduino-serial-communication-f91b95596788) to have an overview of this protocol.**

Implementations are available in various programming languages:

- [Arduino](https://github.com/araffin/arduino-robust-serial)
- [Python](https://github.com/araffin/python-arduino-serial)
- [C++](https://github.com/araffin/cpp-arduino-serial)
- [Rust](https://github.com/araffin/rust-arduino-serial)

## Using the Crate

Please see [Crates.io](https://crates.io/crates/robust-arduino-serial). You need to add only one line to your `Cargo.toml`.

## Tests
Compile and run the tests:
```
cargo test
```

## Documentation

Generate the documentation:
```
cargo doc --open
```

## Examples

Read write in a file
```
cargo run --example file_read_write test.txt
```

Serial communication with an Arduino: [Arduino Source Code](https://github.com/araffin/arduino-robust-serial/tree/master/arduino-board/)
```
cargo run --example arduino_serial /dev/ttyACM0
```

Listen and send orders to the Arduino using threads:
```
cargo run --example arduino_threads /dev/ttyACM0
```
