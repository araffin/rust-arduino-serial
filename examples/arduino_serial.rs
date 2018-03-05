use std::env;

extern crate serial;
extern crate robust_arduino_serial;
use std::time::Duration;
use std::thread;
use serial::prelude::*;
use robust_arduino_serial::*;

// Default settings of Arduino
// see: https://www.arduino.cc/en/Serial/Begin
const SETTINGS: serial::PortSettings = serial::PortSettings {
    baud_rate:    serial::Baud115200,
    char_size:    serial::Bits8,
    parity:       serial::ParityNone,
    stop_bits:    serial::Stop1,
    flow_control: serial::FlowNone,
};


fn main() {

    for arg in env::args_os().skip(1) {
        println!("Opening port: {:?}", arg);
        let mut port = serial::open(&arg).unwrap();
        port.configure(&SETTINGS).unwrap();
        // timeout of 30s
        port.set_timeout(Duration::from_secs(30)).unwrap();

        loop
        {
            println!("Waiting for Arduino...");
            let order = Order::HELLO as i8;
            write_i8(&mut port, order);
            let received_order = convert_i8_to_order(read_i8(&mut port)).unwrap();
            if received_order == Order::ALREADY_CONNECTED
            {
                break;
            }
            thread::sleep(Duration::from_secs(1));
        }

        println!("Connected to Arduino");

        let motor_order = Order::MOTOR as i8;
        let motor_speed: i8 = -56;
        write_i8(&mut port, motor_order);
        write_i8(&mut port, motor_speed);

        write_i8(&mut port, Order::SERVO as i8);
        write_i16(&mut port, 120_i16);

        for _ in 0..10 {
            let order = read_i8(&mut port);
            println!("Ordered received: {:?}", order);

            if let Some(received_order) = convert_i8_to_order(order)
            {
                println!("Known order: {:?}", received_order);
            }
            else
            {
                println!("Unknown order: {:?}", order);
            }
        }
    }
}
