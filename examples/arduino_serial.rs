use std::env;

extern crate serial;
extern crate serial_arduino;
use std::time::Duration;
use serial::prelude::*;
use serial_arduino::*;

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
        port.set_timeout(Duration::from_secs(1)).unwrap();

        let order: i8 = convert_order_to_i8(Order::HELLO);
        write_i8(&mut port, order);

        let motor_order = convert_order_to_i8(Order::MOTOR);
        let motor_speed: i8 = -56;
        write_i8(&mut port, motor_order);
        write_i8(&mut port, motor_speed);

        for _ in 0..2 {
            let order = read_i8(&mut port);
            println!("Ordered received: {:?}", order);

            if let Some(received_order) = convert_i8_to_order(order)
            {
                println!("Known order: {:?}", received_order);
                match received_order
                {
                    Order::MOTOR => {
                        let motor_speed = read_i8(&mut port);
                        println!("Motor Speed = {}", motor_speed);
                    },
                    _ => ()
                }
            }
            else
            {
                println!("Unknown order: {:?}", order);
            }
        }
    }
}
