extern crate std_semaphore;
extern crate serial;
extern crate robust_arduino_serial;
use std::io::ErrorKind;
use std::env;
use std::time::Duration;
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std_semaphore::Semaphore;
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

    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() < 1
    {
        panic!("Please provide a serial port as argument (ex: /dev/ttyACM0)");
    }
    let serial_port = &args[0];

    println!("Opening port: {:?}", serial_port);
    let mut port = serial::open(&serial_port).unwrap();
    port.configure(&SETTINGS).unwrap();
    // timeout of 1ms
    port.set_timeout(Duration::from_millis(1)).unwrap();

    loop
    {
        println!("Waiting for Arduino...");
        let order = Order::HELLO as i8;
        write_i8(&mut port, order).unwrap();
        let received_order = match read_i8(&mut port) {
            Ok(order) => Order::from_i8(order).unwrap(),
            Err(ref e) if e.kind() == ErrorKind::TimedOut => {
                // If we have a read timeout, wait a bit
                thread::sleep(Duration::from_secs(2));
                continue
                }
            Err(e) => {
                    panic!("An error occured reading serial port: {}", e)
                }

        };

        if received_order == Order::ALREADY_CONNECTED
        {
            break;
        }
        thread::sleep(Duration::from_secs(1));
    }

    println!("Connected to Arduino");

    // Channel to send and receive commands
    let (command_sender, command_receiver) = mpsc::channel();
    let command_queue = mpsc::Sender::clone(&command_sender);

    // Wrap the serial to use it in the threads
    let serial_arc = Arc::new(Mutex::new(port));
    let serial_command = serial_arc.clone();

    // Exit event to notify thread when they should exit
    let exit_event = false;
    // Wrap the boolean to use it in the threads
    let exit_arc = Arc::new(Mutex::new(exit_event));
    let exit_listener = exit_arc.clone();
    let exit_command = exit_arc.clone();

    // Semaphore to avoid Arduino buffer overflow:
    // Do not send new order if the Arduino did not aknowledge
    // the previous message
    let n_allowed_messages = 2;
    let semaphore = Arc::new(Semaphore::new(n_allowed_messages));
    let semaphore_command = semaphore.clone();

    let mut threads = vec![];

    // Command thread listen to the command Channel
    // and send orders to the Arduino
    let command_thread = thread::spawn(move || {
        let mut exit = false;
        while !exit
        {
            // Decrement the semaphore counter
            // each time we send an order
            semaphore.acquire();
            let (order, num) = command_receiver.recv().unwrap();

            println!("Sending: {:?}, {}", order, num);

            // Acquire lock on the buffer
            let mut buffer = serial_command.lock().unwrap();

            write_i8(&mut *buffer, order as i8).unwrap();
            match order {
                Order::MOTOR => write_i8(&mut *buffer, num as i8).unwrap(),
                Order::SERVO => write_i16(&mut *buffer, num as i16).unwrap(),
                _ => 0  // Write 0 bytes
            };
            exit = *exit_command.lock().unwrap();
        }
        println!("Command Thread exiting...");
    });

    threads.push(command_thread);

    // Listener thread listens to the Arduino
    // it release the semaphore when an aknowledgement is received
    let listener_thread = thread::spawn(move || {

        let mut exit = false;
        let mut wait = false;
        while !exit
        {
            // Wait a bit so the command thread can acquire the lock
            if wait
            {
                thread::sleep(Duration::from_millis(100));
            }

            // Acquire lock on the serial object
            let mut buffer = serial_arc.lock().unwrap();

            // Receive order from arduino
            let received_order = match read_i8(&mut *buffer) {
                Ok(order) => Order::from_i8(order).unwrap(),
                Err(_) => {
                        wait = true;
                        continue
                    }

            };
            wait = false;

            println!("Received: {:?}", received_order);

            match received_order {
                Order::RECEIVED => semaphore_command.release(),
                _ => ()
            }

            exit = *exit_listener.lock().unwrap();
        }
        println!("Listener Thread exiting...");
    });

    threads.push(listener_thread);

    thread::sleep(Duration::from_secs(1));
    // Send Orders to the Arduino
    command_queue.send((Order::MOTOR, 42_i32)).unwrap();
    command_queue.send((Order::SERVO, 120_i32)).unwrap();

    // Wait a bit before shutting down the threads
    thread::sleep(Duration::from_secs(2));

    // Stop the motor
    command_queue.send((Order::MOTOR, 0_i32)).unwrap();


    // Notify the threads that they should exit
    {
        *exit_arc.lock().unwrap() = true;
    }

    // Send dummy orders to exit the command thread
    command_queue.send((Order::HELLO, 0_i32)).unwrap();

    for t in threads
    {
        t.join().unwrap();
    }
}
