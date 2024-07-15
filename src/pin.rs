use core::marker::PhantomData;

use embedded_hal::digital::{ErrorType, InputPin, OutputPin, StatefulOutputPin};

use crate::{
    error::{Error, ModeChange},
    states, Interface,
};

/// A pin on the SX1509. Use [`into_output`](Self::into_output) or
/// [`into_input`](Self::into_input) to configure the pin as an output or input,
/// respectively.
pub struct Pin<'a, const PIN: u8, I2C> {
    interface: &'a Interface<I2C>,
}

/// An output pin on the SX1509.
pub struct Output<'a, const PIN: u8, I2C, S> {
    pub(crate) interface: &'a Interface<I2C>,
    pub(crate) _state: PhantomData<S>,
}

/// An input pin on the SX1509.
pub struct Input<'a, const PIN: u8, I2C, S, D> {
    pub(crate) interface: &'a Interface<I2C>,
    pub(crate) _state: PhantomData<S>,
    pub(crate) _debounce: PhantomData<D>,
}

impl<'a, const PIN: u8, I2C, E> Pin<'a, PIN, I2C>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
{
    pub(crate) fn new(interface: &'a Interface<I2C>) -> Self {
        Self { interface }
    }

    /// Configure the pin as an output. This will set the pin direction on-chip.
    ///
    /// # Errors
    /// This function will return an error if communication with I2C fails. If
    /// an error occurs, the (unchanged) pin can be extracted from the
    /// [`ModeChange`](ModeChange).
    pub fn into_output(
        self,
    ) -> Result<Output<'a, PIN, I2C, states::PushPull>, ModeChange<Error<E>, Self>> {
        // This will be a lot neater when `try` blocks are stabilized.

        let result = (|| -> Result<(), Error<E>> {
            self.interface.set_output::<PIN>()?;
            self.interface.set_open_drain::<PIN>(false)?;
            Ok(())
        })();

        match result {
            Ok(()) => Ok(Output {
                interface: self.interface,
                _state: PhantomData,
            }),
            Err(error) => Err(ModeChange { error, pin: self }),
        }
    }

    /// Configure the pin as an input. This will set the pin direction on-chip.
    ///
    /// # Errors
    /// This function will return an error if communication with I2C fails. If
    /// an error occurs, the (unchanged) pin can be extracted from the
    /// [`ModeChange`](ModeChange).
    pub fn into_input(
        self,
    ) -> Result<
        Input<'a, PIN, I2C, states::Floating, states::DebounceOff>,
        ModeChange<Error<E>, Self>,
    > {
        // This will be a lot neater when `try` blocks are stabilized.

        let result = (|| -> Result<(), Error<E>> {
            self.interface.set_input::<PIN>()?;
            self.interface.set_pull_up::<PIN>(false)?;
            self.interface.set_pull_down::<PIN>(false)?;
            self.interface.set_debounce_enable::<PIN>(false)?;
            Ok(())
        })();

        match result {
            Ok(()) => Ok(Input {
                interface: self.interface,
                _state: PhantomData,
                _debounce: PhantomData,
            }),
            Err(error) => Err(ModeChange { error, pin: self }),
        }
    }
}

impl<'a, const PIN: u8, I2C, E, S> OutputPin for Output<'a, PIN, I2C, S>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
    E: core::fmt::Debug,
{
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.interface.set_data::<PIN>(false)
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.interface.set_data::<PIN>(true)
    }
}

impl<'a, const PIN: u8, I2C, E, S> StatefulOutputPin for Output<'a, PIN, I2C, S>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
    E: core::fmt::Debug,
{
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        self.interface.get_data::<PIN>()
    }

    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        self.is_set_high().map(|v| !v)
    }
}

impl<'a, const PIN: u8, I2C, E, S, D> InputPin for Input<'a, PIN, I2C, S, D>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
    E: core::fmt::Debug,
{
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        self.interface.get_data::<PIN>()
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        self.is_high().map(|v| !v)
    }
}

impl<'a, const PIN: u8, I2C, E, S> ErrorType for Output<'a, PIN, I2C, S>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
    E: core::fmt::Debug,
{
    type Error = Error<E>;
}

impl<'a, const PIN: u8, I2C, E, S, D> ErrorType for Input<'a, PIN, I2C, S, D>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
    E: core::fmt::Debug,
{
    type Error = Error<E>;
}
