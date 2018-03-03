// extern crate serial;

use std::env;
// use std::time::Duration;

// use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
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

#[derive(Debug)]
enum Order
{
    HELLO,
    MOTOR,
    SERVO
}

fn convert_i8_to_order(order: i8) -> Option<Order>
{
    match order
    {
        0 => Some(Order::HELLO),
        1 => Some(Order::MOTOR),
        2 => Some(Order::SERVO),
        _ => None
    }
}

fn convert_order_to_i8(order: Order) -> i8
{
    match order
    {
        Order::HELLO => 0,
        Order::MOTOR => 1,
        Order::SERVO => 2
    }
}

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


fn read_i8(file: &mut std::fs::File) -> i8
{
    let mut read_buffer = [0u8; 1];
    file.read_exact(&mut read_buffer).unwrap();
    let byte: i8 = unsafe {std::mem::transmute(read_buffer[0])};
    byte
}

fn read_i16(file: &mut std::fs::File) -> i16
{
    let mut read_buffer = [0u8; 2];
    file.read_exact(&mut read_buffer).unwrap();
    let tmp: u16 = ((read_buffer[0] as u16) & 0xff) | ((read_buffer[1] as u16) << 8 & 0xff00);
    let param: i16 = unsafe {std::mem::transmute(tmp)};
    param
}

fn read_i32(file: &mut std::fs::File) -> i32
{
    let mut read_buffer = [0u8; 4];
    file.read_exact(&mut read_buffer).unwrap();
    let tmp: u32 = ((read_buffer[0] as u32) & 0xff) | ((read_buffer[1] as u32) << 8 & 0xff00) | ((read_buffer[2] as u32) << 16 & 0xff0000) | ((read_buffer[3] as u32) << 24 & 0xff000000);
    let param: i32 = unsafe {std::mem::transmute(tmp)};
    param
}

fn write_i8(file: &mut std::fs::File, num: i8)
{
    let buffer = [num as u8];
    file.write(&buffer).unwrap();
}

fn write_i16(file: &mut std::fs::File, num: i16)
{
    let buffer = [
        (num & 0xff) as u8,
        (num >> 8 & 0xff) as u8
    ];
    file.write(&buffer).unwrap();
}

fn write_i32(file: &mut std::fs::File, num: i32)
{
    let buffer = [
        (num & 0xff) as u8,
        (num >> 8 & 0xff) as u8,
        (num >> 16 & 0xff) as u8,
        (num >> 24 & 0xff) as u8
    ];
    file.write(&buffer).unwrap();
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
