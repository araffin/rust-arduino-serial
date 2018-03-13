use std::env;
use std::fs::OpenOptions;
use std::io::SeekFrom;
use std::io::prelude::*;

extern crate robust_arduino_serial;
use robust_arduino_serial::*;


fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 1
    {
        panic!("Please provide a filename as argument");
    }
    let filename = &args[0];
    // Open file and create it if it does not exist
    let mut file = match OpenOptions::new().read(true).write(true).create(true).open(filename)
                    {
                        Err(why) => panic!("Could not open file {}: {}", filename, why),
                        Ok(file) => file
                    };

    let order = Order::HELLO as i8;
    write_i8(&mut file, order);

    let motor_order = Order::MOTOR as i8;
    let motor_speed: i16 = -56;
    write_i8(&mut file, motor_order);
    write_i16(&mut file, motor_speed);
    write_i32(&mut file, 131072);

    // Go to the beginning of the file
    file.seek(SeekFrom::Start(0)).unwrap();

    for _ in 0..2 {
        let order = read_i8(&mut file).unwrap();
        println!("Ordered received: {:?}", order);

        if let Some(received_order) = Order::from_i8(order)
        {
            println!("Known order: {:?}", received_order);
            match received_order
            {
                Order::MOTOR => {
                    let motor_speed = read_i16(&mut file).unwrap();
                    println!("Motor Speed = {}", motor_speed);
                    let test = read_i32(&mut file).unwrap();
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
}
