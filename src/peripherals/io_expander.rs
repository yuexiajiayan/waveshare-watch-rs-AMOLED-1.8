use embedded_hal::digital::{Error as DigitalError, ErrorKind as DigitalErrorKind, ErrorType, OutputPin};
use embedded_hal::i2c::I2c;

use crate::board;

pub struct Tca9554<I> {
    i2c: I,
    config: u8,
    output: u8,
}

impl<I: I2c> Tca9554<I> {
    pub fn new(i2c: I) -> Self {
        Self {
            i2c,
            config: 0xFF,
            output: 0x00,
        }
    }

    pub fn init(&mut self) -> Result<(), I::Error> {
        self.write_reg(0x01, self.output)?;
        self.write_reg(0x03, self.config)
    }

    pub fn into_pin<'a>(&'a mut self, pin: u8, initial_high: bool) -> Result<Tca9554Pin<'a, I>, I::Error> {
        self.set_output(pin, initial_high)?;
        Ok(Tca9554Pin { expander: self, pin })
    }

    pub fn set_output(&mut self, pin: u8, high: bool) -> Result<(), I::Error> {
        let mask = 1u8 << pin;
        self.config &= !mask;
        if high {
            self.output |= mask;
        } else {
            self.output &= !mask;
        }
        self.write_reg(0x01, self.output)?;
        self.write_reg(0x03, self.config)
    }

    fn write_reg(&mut self, reg: u8, val: u8) -> Result<(), I::Error> {
        self.i2c.write(board::EXIO_I2C_ADDR, &[reg, val])
    }
}

#[derive(Debug)]
pub enum Tca9554PinError<E> {
    I2c(E),
}

impl<E: core::fmt::Debug> DigitalError for Tca9554PinError<E> {
    fn kind(&self) -> DigitalErrorKind {
        DigitalErrorKind::Other
    }
}

pub struct Tca9554Pin<'a, I> {
    expander: &'a mut Tca9554<I>,
    pin: u8,
}

impl<I: I2c> ErrorType for Tca9554Pin<'_, I> {
    type Error = Tca9554PinError<I::Error>;
}

impl<I: I2c> OutputPin for Tca9554Pin<'_, I> {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.expander
            .set_output(self.pin, false)
            .map_err(Tca9554PinError::I2c)
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.expander
            .set_output(self.pin, true)
            .map_err(Tca9554PinError::I2c)
    }
}
