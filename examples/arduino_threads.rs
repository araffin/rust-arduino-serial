extern crate std_semaphore;
extern crate serial;
extern crate robust_arduino_serial;
// use std::env;
use std::time::Duration;
use std::thread;
use std::io::Cursor;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std_semaphore::Semaphore;
// use serial::prelude::*;
use robust_arduino_serial::*;
use std::io::Seek;
use std::io::SeekFrom;

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
    let (command_sender, command_receiver) = mpsc::channel();

    let mut buffer = Cursor::new(Vec::new());
    // Write something in the buffer
    write_i8(&mut buffer, Order::RECEIVED as i8);

    let buffer_arc = Arc::new(Mutex::new(buffer));
    let exit_event = false;
    let exit_arc = Arc::new(Mutex::new(exit_event));
    let exit_listener = exit_arc.clone();
    let exit_command = exit_arc.clone();

    let semaphore = Arc::new(Semaphore::new(0));
    let semaphore_command = semaphore.clone();
    let buffer_command = buffer_arc.clone();
    let mut threads = vec![];

    let command_queue = mpsc::Sender::clone(&command_sender);
    let command_thread = thread::spawn(move || {
        let mut exit = false;
        while !exit
        {
            for _ in 0..2 {
                let (order, num) = command_receiver.recv().unwrap();
                println!("Got: {:?}, {}", order, num);
                // Acquire lock on the buffer
                let mut buff = buffer_command.lock().unwrap();
                write_i8(&mut *buff, order as i8);
                match order {
                    Order::MOTOR => write_i8(&mut *buff, num as i8),
                    Order::SERVO => write_i16(&mut *buff, num as i16),
                    _ => ()
                }
            }
            exit = *exit_command.lock().unwrap();
        }
    });

    threads.push(command_thread);

    let listener_thread = thread::spawn(move || {

        let mut exit = false;
        while !exit
        {
            command_sender.send((Order::SERVO, 16_i32)).unwrap();

            thread::sleep(Duration::from_secs(1));
            // Acquire lock on the buffer
            let mut buff = buffer_arc.lock().unwrap();

            // Go to the beginning of the buffer
            (*buff).seek(SeekFrom::Start(0)).unwrap();

            // Receive order from arduino
            let received_order = convert_i8_to_order(read_i8(&mut *buff)).unwrap();

            match received_order {
                Order::RECEIVED => semaphore_command.release(),
                _ => ()
            }
            exit = *exit_listener.lock().unwrap();
        }

    });

    threads.push(listener_thread);

    // Set exit event to true
    {
        *exit_arc.lock().unwrap() = true;
    }

    semaphore.acquire();
    command_queue.send((Order::MOTOR, 42_i32)).unwrap();

    for t in threads
    {
        t.join().unwrap();
    }
    //
    // for arg in env::args_os().skip(1) {
    //     println!("Opening port: {:?}", arg);
    //     let mut port = serial::open(&arg).unwrap();
    //     port.configure(&SETTINGS).unwrap();
    //     // timeout of 30s
    //     port.set_timeout(Duration::from_secs(30)).unwrap();
    //
    //     loop
    //     {
    //         println!("Waiting for Arduino...");
    //         let order = Order::HELLO as i8;
    //         write_i8(&mut port, order);
    //         let received_order = convert_i8_to_order(read_i8(&mut port)).unwrap();
    //         if received_order == Order::ALREADY_CONNECTED
    //         {
    //             break;
    //         }
    //         thread::sleep(Duration::from_secs(1));
    //     }
    //
    //     println!("Connected to Arduino");
    //
    //     let motor_order = Order::MOTOR as i8;
    //     let motor_speed: i8 = -56;
    //     write_i8(&mut port, motor_order);
    //     write_i8(&mut port, motor_speed);
    //
    //     write_i8(&mut port, Order::SERVO as i8);
    //     write_i16(&mut port, 120_i16);
    //
    //     for _ in 0..10 {
    //         let order = read_i8(&mut port);
    //         println!("Order received: {:?}", order);
    //
    //         if let Some(received_order) = convert_i8_to_order(order)
    //         {
    //             println!("Known order: {:?}", received_order);
    //         }
    //         else
    //         {
    //             println!("Unknown order: {:?}", order);
    //         }
    //     }
    // }
}
