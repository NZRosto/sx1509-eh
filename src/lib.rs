#![doc = include_str!("../README.md")]
#![no_std]

pub use interface::DebounceTime;
use interface::Interface;
pub use pin::{Input, Output, Pin};

mod interface;
mod pin;
mod reg;

/// Error types.
pub mod error;
/// State types for the pins.
pub mod states;

/// The SX1509 driver. Use [`new`](Self::new) to create a new instance of the
/// driver, and then [`split`](Self::split) to get individual pins that support
/// the [`embedded_hal`] traits.
pub struct Sx1509<I2C> {
    interface: Interface<I2C>,
}

impl<I2C, E> Sx1509<I2C>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
{
    /// Create a new instance of the SX1509 driver. This performs a reset of the
    /// device and may fail if the device is not present.
    ///
    /// # Errors
    /// This function will return an error if communication with I2C fails for
    /// any reason.
    pub fn new(mut i2c: I2C, address: u8) -> Result<Self, E> {
        // Reset the device.
        i2c.write(address, &[reg::Register::RegReset as u8, 0x12])?;
        i2c.write(address, &[reg::Register::RegReset as u8, 0x34])?;

        // Enable internal 2MHz oscillator.
        i2c.write(address, &[reg::Register::RegClock as u8, 0b0100_0000])?;

        Ok(Self {
            interface: Interface::new(spin::Mutex::new(i2c), address),
        })
    }

    /// Set the debounce time for the expander. This will affect all pins on the
    /// chip.
    ///
    /// # Errors
    /// This function will return an error if communication with I2C fails.
    pub fn set_debounce_time(
        &mut self,
        debounce_time: DebounceTime,
    ) -> Result<(), error::Error<E>> {
        self.interface.set_debounce_time(debounce_time)
    }

    /// Split the expander into individual pins. This allows you to configure
    /// each pin as an input or output. A mutable reference is used to ensure
    /// multiple sets of pins cannot exist at the same time.
    pub fn split(&mut self) -> Pins<'_, I2C> {
        Pins {
            a0: Pin::new(&self.interface),
            a1: Pin::new(&self.interface),
            a2: Pin::new(&self.interface),
            a3: Pin::new(&self.interface),
            a4: Pin::new(&self.interface),
            a5: Pin::new(&self.interface),
            a6: Pin::new(&self.interface),
            a7: Pin::new(&self.interface),

            b0: Pin::new(&self.interface),
            b1: Pin::new(&self.interface),
            b2: Pin::new(&self.interface),
            b3: Pin::new(&self.interface),
            b4: Pin::new(&self.interface),
            b5: Pin::new(&self.interface),
            b6: Pin::new(&self.interface),
            b7: Pin::new(&self.interface),
        }
    }
}

/// The pins on the SX1509.
pub struct Pins<'a, I2C> {
    /// Bank A, Pin 0
    pub a0: Pin<'a, 0, I2C>,
    /// Bank A, Pin 1
    pub a1: Pin<'a, 1, I2C>,
    /// Bank A, Pin 2
    pub a2: Pin<'a, 2, I2C>,
    /// Bank A, Pin 3
    pub a3: Pin<'a, 3, I2C>,
    /// Bank A, Pin 4
    pub a4: Pin<'a, 4, I2C>,
    /// Bank A, Pin 5
    pub a5: Pin<'a, 5, I2C>,
    /// Bank A, Pin 6
    pub a6: Pin<'a, 6, I2C>,
    /// Bank A, Pin 7
    pub a7: Pin<'a, 7, I2C>,

    /// Bank B, Pin 0
    pub b0: Pin<'a, 8, I2C>,
    /// Bank B, Pin 1
    pub b1: Pin<'a, 9, I2C>,
    /// Bank B, Pin 2
    pub b2: Pin<'a, 10, I2C>,
    /// Bank B, Pin 3
    pub b3: Pin<'a, 11, I2C>,
    /// Bank B, Pin 4
    pub b4: Pin<'a, 12, I2C>,
    /// Bank B, Pin 5
    pub b5: Pin<'a, 13, I2C>,
    /// Bank B, Pin 6
    pub b6: Pin<'a, 14, I2C>,
    /// Bank B, Pin 7
    pub b7: Pin<'a, 15, I2C>,
}
