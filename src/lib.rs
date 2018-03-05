//! # Robust Arduino Serial
//!
//! `robust_arduino_serial` is a simple and robust serial communication protocol.
//! It was designed to make two arduinos communicate, but can also be useful
//! when you want a computer (e.g. a Raspberry Pi) to communicate with an Arduino.
//!
#[derive(Debug, PartialEq, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum Order
{
    HELLO = 0,
    MOTOR = 1,
    SERVO = 2,
    ALREADY_CONNECTED = 3,
    ERROR = 4,
    RECEIVED = 5,
    STOP = 6
}

/// Convert an int to an Order
///
/// # Example
///
/// ```
/// use robust_arduino_serial::Order;
///
/// let order: i8 = 1;  // Order::MOTOR has the index 1 in the enum
/// let converted_order = robust_arduino_serial::convert_i8_to_order(order).unwrap();
///
/// assert_eq!(converted_order, Order::MOTOR);
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

/// Read one byte from a file/serial port and convert it to a 8 bits int
///
/// # Example
///
/// ```
/// use std::io::Cursor;
///
/// let mut buffer = Cursor::new(vec![2]);
/// let num: i8 = robust_arduino_serial::read_i8(&mut buffer);
///
/// assert_eq!(2, num);
/// ```
pub fn read_i8<T: std::io::Read>(file: &mut T) -> i8
{
    let mut read_buffer = [0u8; 1];
    file.read_exact(&mut read_buffer).unwrap();
    read_buffer[0] as i8
}

/// Read two bytes from a file/serial port and convert it to a 16 bits int
///
/// # Example
///
/// ```
/// use std::io::Cursor;
/// use std::io::SeekFrom;
/// use std::io::prelude::*;
/// use robust_arduino_serial::*;
///
/// let mut buffer = Cursor::new(Vec::new());
/// let number: i16 = -355;
///
/// // Write the number to the buffer
/// write_i16(&mut buffer, number);
///
/// // Go to the beginning of the buffer
/// buffer.seek(SeekFrom::Start(0)).unwrap();
///
/// // Read 16 bits (two bytes) from the buffer
/// let read_number: i16 = robust_arduino_serial::read_i16(&mut buffer);
///
/// assert_eq!(read_number, number);
/// ```
pub fn read_i16<T: std::io::Read>(file: &mut T) -> i16
{
    let mut read_buffer = [0u8; 2];
    file.read_exact(&mut read_buffer).unwrap();
    let number: u16 = ((read_buffer[0] as u16) & 0xff) | ((read_buffer[1] as u16) << 8 & 0xff00);
    number as i16
}

/// Read four bytes from a file/serial port and convert it to a 32 bits int
///
/// # Example
///
/// ```
/// use std::io::Cursor;
/// use std::io::SeekFrom;
/// use std::io::prelude::*;
/// use robust_arduino_serial::*;
///
/// let mut buffer = Cursor::new(Vec::new());
/// let big_number: i32 = 16384; // 2^14
///
/// // Write the number to the buffer
/// write_i32(&mut buffer, big_number);
///
/// // Go to the beginning of the buffer
/// buffer.seek(SeekFrom::Start(0)).unwrap();
///
/// // Read 32 bits (four bytes) from the buffer
/// let read_number: i32 = robust_arduino_serial::read_i32(&mut buffer);
///
/// assert_eq!(big_number, read_number);
/// ```
pub fn read_i32<T: std::io::Read>(file: &mut T) -> i32
{
    let mut read_buffer = [0u8; 4];
    file.read_exact(&mut read_buffer).unwrap();
    let number: u32 = ((read_buffer[0] as u32) & 0xff) | ((read_buffer[1] as u32) << 8 & 0xff00) | ((read_buffer[2] as u32) << 16 & 0xff0000) | ((read_buffer[3] as u32) << 24 & 0xff000000);
    number as i32
}

/// Write one byte int to a file/serial port
///
/// # Example
///
/// ```
/// let mut buffer = Vec::new();
/// let num: i8 = 2;
///
/// // write 8 bits (one byte) to the buffer
/// robust_arduino_serial::write_i8(&mut buffer, num);
/// ```
pub fn write_i8<T: std::io::Write>(file: &mut T, num: i8)
{
    let buffer = [num as u8];
    file.write(&buffer).unwrap();
}

/// Write two bytes int to a file/serial port
///
/// # Example
///
/// ```
/// use std::io::Cursor;
/// use robust_arduino_serial::*;
///
/// let mut buffer = Cursor::new(Vec::new());
/// let number: i16 = 366;
///
/// // write 16 bits (two bytes) to the buffer
/// write_i16(&mut buffer, number);
/// ```
pub fn write_i16<T: std::io::Write>(file: &mut T, num: i16)
{
    let buffer = [
        (num & 0xff) as u8,
        (num >> 8 & 0xff) as u8
    ];
    file.write(&buffer).unwrap();
}

/// Write four bytes int to a file/serial port
///
/// # Example
///
/// ```
/// use std::io::Cursor;
/// use robust_arduino_serial::*;
///
/// let mut buffer = Cursor::new(Vec::new());
/// let big_number: i32 = -16384; // -2^14
///
/// // write 32 bits (four bytes) to the buffer
/// write_i32(&mut buffer, big_number);
/// ```
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
    use std::io::Cursor;
    use std::io::SeekFrom;
    use std::io::prelude::*;

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
            assert_eq!(convert_i8_to_order(i as i8).unwrap(), *order);
        }
    }

    #[test]
    fn read_write_orders() {
        let motor_speed: i8 = -57;
        let servo_angle: i16 = 512; // 2^9
        let big_number: i32 = -32768; // -2^15

        let mut buffer = Cursor::new(Vec::new());

        write_i8(&mut buffer, Order::MOTOR as i8);
        write_i8(&mut buffer, motor_speed);

        write_i8(&mut buffer, Order::SERVO as i8);
        write_i16(&mut buffer, servo_angle);

        write_i8(&mut buffer, Order::ERROR as i8);
        write_i32(&mut buffer, big_number);

        // Go to the beginning of the buffer
        buffer.seek(SeekFrom::Start(0)).unwrap();

        let read_1st_order = convert_i8_to_order(read_i8(&mut buffer)).unwrap();
        let read_motor_speed = read_i8(&mut buffer);

        let read_2nd_order = convert_i8_to_order(read_i8(&mut buffer)).unwrap();
        let read_servo_angle = read_i16(&mut buffer);

        let read_3rd_order = convert_i8_to_order(read_i8(&mut buffer)).unwrap();
        let read_big_number = read_i32(&mut buffer);

        assert_eq!(read_1st_order, Order::MOTOR);
        assert_eq!(read_motor_speed, motor_speed);

        assert_eq!(read_2nd_order, Order::SERVO);
        assert_eq!(read_servo_angle, servo_angle);

        assert_eq!(read_3rd_order, Order::ERROR);
        assert_eq!(read_big_number, big_number);
    }
}
