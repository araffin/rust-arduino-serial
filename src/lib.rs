//! # Serial Arduino
//!
//! `serial_arduino` is a simple and robust serial communication protocol.
//! It was designed to make two arduinos communicate, but can also be useful
//! when you want a computer (e.g. a Raspberry Pi) to communicate with an Arduino.
//!
#[derive(Debug, PartialEq, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum Order
{
    HELLO,
    MOTOR,
    SERVO,
    ALREADY_CONNECTED,
    ERROR,
    RECEIVED,
    STOP
}

/// Convert an int to an Order
///
/// # Examples
///
/// ```
/// let order: i8 = 1;
/// // converted_order = Order::MOTOR
/// let converted_order = serial_arduino::convert_i8_to_order(order).unwrap();
/// ```
pub fn convert_i8_to_order(order: i8) -> Option<Order>
{
    match order
    {
        0 => Some(Order::HELLO),
        1 => Some(Order::MOTOR),
        2 => Some(Order::SERVO),
        3 => Some(Order::ALREADY_CONNECTED),
        4 => Some(Order::ERROR),
        5 => Some(Order::RECEIVED),
        6 => Some(Order::STOP),
        _ => None
    }
}

pub fn convert_order_to_i8(order: Order) -> i8
{
    match order
    {
        Order::HELLO => 0,
        Order::MOTOR => 1,
        Order::SERVO => 2,
        Order::ALREADY_CONNECTED => 3,
        Order::ERROR => 4,
        Order::RECEIVED => 5,
        Order::STOP => 6
    }
}

pub fn read_i8<T: std::io::Read>(file: &mut T) -> i8
{
    let mut read_buffer = [0u8; 1];
    file.read_exact(&mut read_buffer).unwrap();
    let byte: i8 = unsafe {std::mem::transmute(read_buffer[0])};
    byte
}

pub fn read_i16<T: std::io::Read>(file: &mut T) -> i16
{
    let mut read_buffer = [0u8; 2];
    file.read_exact(&mut read_buffer).unwrap();
    let tmp: u16 = ((read_buffer[0] as u16) & 0xff) | ((read_buffer[1] as u16) << 8 & 0xff00);
    let param: i16 = unsafe {std::mem::transmute(tmp)};
    param
}

pub fn read_i32<T: std::io::Read>(file: &mut T) -> i32
{
    let mut read_buffer = [0u8; 4];
    file.read_exact(&mut read_buffer).unwrap();
    let tmp: u32 = ((read_buffer[0] as u32) & 0xff) | ((read_buffer[1] as u32) << 8 & 0xff00) | ((read_buffer[2] as u32) << 16 & 0xff0000) | ((read_buffer[3] as u32) << 24 & 0xff000000);
    let param: i32 = unsafe {std::mem::transmute(tmp)};
    param
}

pub fn write_i8<T: std::io::Write>(file: &mut T, num: i8)
{
    let buffer = [num as u8];
    file.write(&buffer).unwrap();
}

pub fn write_i16<T: std::io::Write>(file: &mut T, num: i16)
{
    let buffer = [
        (num & 0xff) as u8,
        (num >> 8 & 0xff) as u8
    ];
    file.write(&buffer).unwrap();
}

pub fn write_i32<T: std::io::Write>(file: &mut T, num: i32)
{
    let buffer = [
        (num & 0xff) as u8,
        (num >> 8 & 0xff) as u8,
        (num >> 16 & 0xff) as u8,
        (num >> 24 & 0xff) as u8
    ];
    file.write(&buffer).unwrap();
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::fs::File;
    use std::fs::OpenOptions;

    // Create a file
    fn create_file() -> std::fs::File
    {
        let filename = "test_file.txt";
        let file = match OpenOptions::new().write(true).create(true).open(filename)
        {
            Err(why) => panic!("Could not open file {}: {}", filename, why),
            Ok(file) => file
        };
        file
    }

    fn remove_test_file()
    {
        match fs::remove_file("test_file.txt")
        {
            Err(why) => panic!("Could not remove test file: {}", why),
            Ok(_) => ()
        };
    }

    #[test]
    fn create_delete_test_file() {
        create_file();
        remove_test_file();
    }

    #[test]
    fn test_order_conversion()
    {
        use Order::*;
        let orders: [Order; 7] = [
            HELLO,
            MOTOR,
            SERVO,
            ALREADY_CONNECTED,
            ERROR,
            RECEIVED,
            STOP
        ];

        for (i, order) in orders.iter().enumerate()
        {
            assert_eq!(i as i8, convert_order_to_i8(*order));
            assert_eq!(convert_i8_to_order(i as i8).unwrap(), *order);
        }
    }

    #[test]
    fn write_order() {
        let motor_speed: i8 = -57;
        let servo_angle: i16 = 512; // 2^9
        let mut file = create_file();
        write_i8(&mut file, convert_order_to_i8(Order::MOTOR));
        write_i8(&mut file, motor_speed);

        write_i8(&mut file, convert_order_to_i8(Order::SERVO));
        write_i16(&mut file, servo_angle);

        let mut f = File::open("test_file.txt").unwrap();

        let read_1st_order = convert_i8_to_order(read_i8(&mut f)).unwrap();
        let read_motor_speed = read_i8(&mut f);

        let read_2nd_order = convert_i8_to_order(read_i8(&mut f)).unwrap();
        let read_servo_angle = read_i16(&mut f);

        assert_eq!(read_1st_order, Order::MOTOR);
        assert_eq!(read_motor_speed, motor_speed);

        assert_eq!(read_2nd_order, Order::SERVO);
        assert_eq!(read_servo_angle, servo_angle);

        remove_test_file();
    }
}
