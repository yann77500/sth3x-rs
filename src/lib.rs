#![no_std]
#![no_main]

use crc::{Crc, CRC_8_NRSC_5};
use embedded_hal::blocking::i2c::{Read, Write, WriteRead};

#[derive(Debug)]
pub enum Error<E> {
    I2C(E),
    CRC_ERROR,
}

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

    fn convert_temperature_degrees(&self, rd_buff: &[u8; 2]) -> f32 {
        let temperature_raw: u16 = (rd_buff[0] as u16) << 8 | rd_buff[1] as u16;

        let mut temperature: f32 = temperature_raw as f32 / (65535.0);
        temperature *= 175.0;
        temperature -= 45.0;

        return temperature;
    }

    fn convert_hygro(&self, rd_buff: &[u8; 2]) -> f32 {
        let hygro_raw = (rd_buff[0] as u16) << 8 | rd_buff[1] as u16;
        let hygro = 100.0 * ((hygro_raw as f32) / 65535.0);

        return hygro;
    }

    ///
    ///
    ///
    pub fn read_temperature_humidity(&mut self) -> Result<(f32, f32), Error<E>> {
        let mut rd_buff: [u8; 6] = [0; 6];

        match self.i2c.write_read(self.addr, &[0x2C, 0x06], &mut rd_buff) {
            Ok(()) => {
                let temp_bytes = &[rd_buff[0], rd_buff[1]];
                let hygro_bytes = &[rd_buff[3], rd_buff[4]];

                let crc = Crc::<u8>::new(&CRC_8_NRSC_5);
                let mut digest = crc.digest();
                digest.update(temp_bytes);

                if digest.finalize() != rd_buff[2] {
                    return Err(Error::CRC_ERROR);
                }

                //Convertir la temperature en degrees
                let temperature = self.convert_temperature_degrees(&temp_bytes);

                let mut digest = crc.digest();
                digest.update(hygro_bytes);

                if digest.finalize() != rd_buff[5] {
                    return Err(Error::CRC_ERROR);
                }

                //Convertir l hygrometrie
                let hygro = self.convert_hygro(hygro_bytes);

                return Ok((temperature, hygro));
            }
            Err(e) => {
                return Err(Error::I2C(e));
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
