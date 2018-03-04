use std::io::prelude::*;

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

pub fn read_i8(file: &mut std::fs::File) -> i8
{
    let mut read_buffer = [0u8; 1];
    file.read_exact(&mut read_buffer).unwrap();
    let byte: i8 = unsafe {std::mem::transmute(read_buffer[0])};
    byte
}

pub fn read_i16(file: &mut std::fs::File) -> i16
{
    let mut read_buffer = [0u8; 2];
    file.read_exact(&mut read_buffer).unwrap();
    let tmp: u16 = ((read_buffer[0] as u16) & 0xff) | ((read_buffer[1] as u16) << 8 & 0xff00);
    let param: i16 = unsafe {std::mem::transmute(tmp)};
    param
}

pub fn read_i32(file: &mut std::fs::File) -> i32
{
    let mut read_buffer = [0u8; 4];
    file.read_exact(&mut read_buffer).unwrap();
    let tmp: u32 = ((read_buffer[0] as u32) & 0xff) | ((read_buffer[1] as u32) << 8 & 0xff00) | ((read_buffer[2] as u32) << 16 & 0xff0000) | ((read_buffer[3] as u32) << 24 & 0xff000000);
    let param: i32 = unsafe {std::mem::transmute(tmp)};
    param
}

pub fn write_i8(file: &mut std::fs::File, num: i8)
{
    let buffer = [num as u8];
    file.write(&buffer).unwrap();
}

pub fn write_i16(file: &mut std::fs::File, num: i16)
{
    let buffer = [
        (num & 0xff) as u8,
        (num >> 8 & 0xff) as u8
    ];
    file.write(&buffer).unwrap();
}

pub fn write_i32(file: &mut std::fs::File, num: i32)
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
    use std::fs::OpenOptions;
    use std::path::Path;

    // Create a file
    fn create_file() -> std::fs::File
    {
        let filename = "test_file.txt";
        if Path::new(filename).exists()
        {
            remove_test_file();
        }
        let file = match OpenOptions::new().read(true).write(true).create(true).open(filename)
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
        create_file();
        remove_test_file();
    }
}
