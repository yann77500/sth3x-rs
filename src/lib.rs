#![no_std]
#![no_main]

use embedded_hal::blocking::i2c::{Read, Write, WriteRead};


#[derive(Debug)]
pub struct sth3x<I2C> {
    i2c: I2C,
    addr: u8,

}


const ADDR_DEFAULT: u8 = 0x44;

impl<I2C, E> sth3x<I2C>
    where
        I2C: Read<Error = E> + Write<Error = E> + WriteRead<Error = E>,
{


}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
