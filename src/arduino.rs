extern crate serial;

use std;
use self::serial::prelude::*;


pub fn read_i8(port: &mut SerialPort) -> i8
{
    let mut read_buffer = [0u8; 1];
    port.read_exact(&mut read_buffer).unwrap();
    let byte: i8 = unsafe {std::mem::transmute(read_buffer[0])};
    byte
}


pub fn read_i16(port: &mut SerialPort) -> i16
{
    let mut read_buffer = [0u8; 2];
    port.read_exact(&mut read_buffer).unwrap();
    let tmp: u16 = ((read_buffer[0] as u16) & 0xff) | ((read_buffer[1] as u16) << 8 & 0xff00);
    let param: i16 = unsafe {std::mem::transmute(tmp)};
    param
}

pub fn read_i32(port: &mut SerialPort) -> i32
{
    let mut read_buffer = [0u8; 4];
    port.read_exact(&mut read_buffer).unwrap();
    let tmp: u32 = ((read_buffer[0] as u32) & 0xff) | ((read_buffer[1] as u32) << 8 & 0xff00) | ((read_buffer[2] as u32) << 16 & 0xff0000) | ((read_buffer[3] as u32) << 24 & 0xff000000);
    let param: i32 = unsafe {std::mem::transmute(tmp)};
    param
}

pub fn write_i8(port: &mut SerialPort, num: i8)
{
    let buffer = [num as u8];
    port.write(&buffer).unwrap();
}

pub fn write_i16(port: &mut SerialPort, num: i16)
{
    let buffer = [
        (num & 0xff) as u8,
        (num >> 8 & 0xff) as u8
    ];
    port.write(&buffer).unwrap();
}

pub fn write_i32(port: &mut SerialPort, num: i32)
{
    let buffer = [
        (num & 0xff) as u8,
        (num >> 8 & 0xff) as u8,
        (num >> 16 & 0xff) as u8,
        (num >> 24 & 0xff) as u8
    ];
    port.write(&buffer).unwrap();
}
