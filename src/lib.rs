#![no_std]
#![no_main]

use embedded_hal::blocking::i2c::{Read, Write, WriteRead};

/// Repeatability of measurement
///     High => Difference between 2 measurement is lowest but time to measurement is highest
///     Medium =>
///     Low => Difference between 2 measurement is highest but time to measurement is lowest
pub enum Repeatability {
    LOW,
    MEDIUM,
    HIGH,
}

pub enum ClockStretching {
    ClkStretchingOn = 0x2C,
    ClkStretchingOff = 0x24,
}

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
    pub fn new_default(i2c: I2C) -> Self {
        sth3x {
            i2c,
            addr: ADDR_DEFAULT,
        }
    }

    pub fn new(i2c: I2C, addr: u8) -> Self {
        sth3x { i2c, addr }
    }

    ///
    ///
    ///
    pub fn read_temperature_humidity(&mut self) -> Result<(f32, f32), E> {
        let mut rd_buff: [u8; 6] = [0; 6];

        match self.i2c.write_read(self.addr, &[0x2C, 0x06], &mut rd_buff) {
            Ok(()) => {
                //TODO : Methode de convertion temperature du buff vers f32
                let temperature_raw: u16 = (rd_buff[0] as u16) << 8 | rd_buff[1] as u16;

                let mut temperature: f32 = temperature_raw as f32 / (65535.0);
                temperature *= 175.0;
                temperature -= 45.0;

                //TODO : Methode de convertion hygro du buff vers f32
                let hygro_raw = (rd_buff[3] as u16) << 8 | rd_buff[4] as u16;
                let hygro = 100.0 * ((hygro_raw as f32) / 65535.0);

                return Ok((temperature, hygro));
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
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
