# Robust Arduino Serial Protocol in Rust

[![Build Status](https://travis-ci.org/araffin/rust-arduino-serial.svg?branch=master)](https://travis-ci.org/araffin/rust-arduino-serial) [![Build status](https://ci.appveyor.com/api/projects/status/h0ejgesat0nnpahc/branch/master?svg=true)](https://ci.appveyor.com/project/araffin/rust-arduino-serial/branch/master)

`robust_arduino_serial` is a simple and robust serial communication protocol. It was designed to make two arduinos communicate, but can also be useful when you want a computer (e.g. a Raspberry Pi) to communicate with an Arduino.

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

Serial communication with an Arduino: [Arduino Source Code](https://github.com/sergionr2/RacingRobot/tree/master/arduino)
```
cargo run --example arduino_serial /dev/ttyACM0
```
