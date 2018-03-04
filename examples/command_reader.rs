use std::env;
use std::fs::OpenOptions;
// extern crate serial;

extern crate serial_arduino;
use serial_arduino::*;

// use serial::prelude::*;
//
// // Default settings of Arduino
// // see: https://www.arduino.cc/en/Serial/Begin
// const SETTINGS: serial::PortSettings = serial::PortSettings {
//     baud_rate:    serial::Baud115200,
//     char_size:    serial::Bits8,
//     parity:       serial::ParityNone,
//     stop_bits:    serial::Stop1,
//     flow_control: serial::FlowNone,
// };


fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let filename = &args[0];
    // Open file and create it if it does not exist
    let mut file = match OpenOptions::new().read(true).write(true).create(true).open(filename)
                    {
                        Err(why) => panic!("Could not open file {}: {}", filename, why),
                        Ok(file) => file
                    };

    for _ in 0..2 {
        let order = read_i8(&mut file);
        println!("Ordered received: {:?}", order);

        if let Some(my_order) = convert_i8_to_order(order)
        {
            println!("Known order: {:?}", my_order);
            match my_order
            {
                Order::MOTOR => {
                    let motor_speed = read_i16(&mut file);
                    println!("Motor Speed = {}", motor_speed);
                    let test = read_i32(&mut file);
                    println!("test = {}", test);
                },
                _ => ()
            }
        }
        else
        {
            println!("Unknown order: {:?}", order);
        }
    }

    let order: i8 = convert_order_to_i8(Order::HELLO);
    write_i8(&mut file, order);

    let motor_order = convert_order_to_i8(Order::MOTOR);
    let motor_speed: i16 = -56;
    write_i8(&mut file, motor_order);
    write_i16(&mut file, motor_speed);
    write_i32(&mut file, 131072);

    // for arg in env::args_os().skip(1) {
    //     println!("opening port: {:?}", arg);
    //     let mut port = serial::open(&arg).unwrap();
    //
    //     interact(&mut port).unwrap();
    // }
}


//
// fn interact<T: SerialPort>(port: &mut T) -> serial::Result<()> {
//     try!(port.configure(&SETTINGS));
//     try!(port.set_timeout(Duration::from_secs(1)));
//
//     let mut buf: Vec<u8> = (0..255).collect();
//
//     println!("writing bytes");
//     try!(port.write(&buf[..]));
//
//     println!("reading bytes");
//     try!(port.read(&mut buf[..]));
//
//     Ok(())
// }
//
// fn readOneByte<T: SerialPort>(port: &mut T) -> i8
// {
//
// }
