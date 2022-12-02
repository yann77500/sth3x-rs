# sth3x-rs
(At 02/12/2022 In progress) 

Project to manage an STH3x temperature and humidity sensor family 

This driver use I2C embedded-hal  implementation. 

See example of implementation (tested on STM32WB55) :


    let mut sda = gpio::Pin::new(Port::B, 9, PinMode::Alt(4)); //PB9 as I2C1 SDA
    let mut scl = gpio::Pin::new(Port::B, 8, PinMode::Alt(4)); //PB9 as I2C1 SCL
    sda.pull(Pull::Up);
    scl.pull(Pull::Up);

    let i2cConfig: I2cConfig = I2cConfig::default(); 
    let mut i2c = stm32_hal2::i2c::I2c::<stm32_hal2::pac::I2C1>::new(dp.I2C1, i2cConfig, &clocks);


    let sth3x_sensor = sth3x::sth3x::new_default(i2c)

    .....
    ....
    match sth3x_sensor.read_temperature_humidity() {
        Ok((temp,hygro)) => {
    
        }
        _ => {
        }
    }

Pleases refer to the following project to see more examples : 


