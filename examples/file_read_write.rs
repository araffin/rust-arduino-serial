use std::env;
use std::fs::File;
use std::fs::OpenOptions;

extern crate serial_arduino;
use serial_arduino::*;


fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let filename = &args[0];
    // Open file and create it if it does not exist
    let mut file = match OpenOptions::new().write(true).create(true).open(filename)
                    {
                        Err(why) => panic!("Could not open file {}: {}", filename, why),
                        Ok(file) => file
                    };

    let order: i8 = convert_order_to_i8(Order::HELLO);
    write_i8(&mut file, order);

    let motor_order = convert_order_to_i8(Order::MOTOR);
    let motor_speed: i16 = -56;
    write_i8(&mut file, motor_order);
    write_i16(&mut file, motor_speed);
    write_i32(&mut file, 131072);

    let mut read_file = File::open(filename).unwrap();

    for _ in 0..2 {
        let order = read_i8(&mut read_file);
        println!("Ordered received: {:?}", order);

        if let Some(received_order) = convert_i8_to_order(order)
        {
            println!("Known order: {:?}", received_order);
            match received_order
            {
                Order::MOTOR => {
                    let motor_speed = read_i16(&mut read_file);
                    println!("Motor Speed = {}", motor_speed);
                    let test = read_i32(&mut read_file);
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
