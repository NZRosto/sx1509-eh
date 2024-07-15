use core::marker::PhantomData;

use crate::{
    error::{Error, ModeChange},
    Input, Output,
};

/// A push-pull output.
pub struct PushPull;
/// An open-drain output.
pub struct OpenDrain;
/// A pull-up input.
pub struct PullUp;
/// A pull-down input.
pub struct PullDown;
/// A floating input.
pub struct Floating;
/// A debounced input.
pub struct DebounceOn;
/// A non-debounced input.
pub struct DebounceOff;

impl<'a, const PIN: u8, I2C, E, S, D> Input<'a, PIN, I2C, S, D>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
{
    /// Configure the pin as an output.
    ///
    /// # Errors
    /// See [`Pin::into_output`](crate::Pin::into_output).
    pub fn into_output(self) -> Result<Output<'a, PIN, I2C, PushPull>, ModeChange<Error<E>, Self>> {
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
}

impl<'a, const PIN: u8, I2C, E, S> Output<'a, PIN, I2C, S>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
{
    /// Configure the pin as an input.
    ///
    /// # Errors
    /// See [`Pin::into_input`](crate::Pin::into_input).
    pub fn into_input(
        self,
    ) -> Result<Input<'a, PIN, I2C, Floating, DebounceOff>, ModeChange<Error<E>, Self>> {
        let result = (|| -> Result<(), Error<E>> {
            self.interface.set_input::<PIN>()?;
            self.interface.set_pull_up::<PIN>(false)?;
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

impl<'a, const PIN: u8, I2C, E, D> Input<'a, PIN, I2C, Floating, D>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
{
    /// Configure the pin as a pull-up input.
    ///
    /// # Errors
    /// See [`Pin::into_input`](crate::Pin::into_input).
    pub fn pullup(self) -> Result<Input<'a, PIN, I2C, PullUp, D>, ModeChange<Error<E>, Self>> {
        match self.interface.set_pull_up::<PIN>(true) {
            Ok(()) => Ok(Input {
                interface: self.interface,
                _state: PhantomData,
                _debounce: PhantomData,
            }),
            Err(error) => Err(ModeChange { error, pin: self }),
        }
    }

    /// Configure the pin as a pull-down input.
    ///
    /// # Errors
    /// See [`Pin::into_input`](crate::Pin::into_input).
    pub fn pulldown(self) -> Result<Input<'a, PIN, I2C, PullDown, D>, ModeChange<Error<E>, Self>> {
        match self.interface.set_pull_down::<PIN>(true) {
            Ok(()) => Ok(Input {
                interface: self.interface,
                _state: PhantomData,
                _debounce: PhantomData,
            }),
            Err(error) => Err(ModeChange { error, pin: self }),
        }
    }
}

impl<'a, const PIN: u8, I2C, E, D> Input<'a, PIN, I2C, PullUp, D>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
{
    /// Configure the pin as a floating input.
    ///
    /// # Errors
    /// See [`Pin::into_input`](crate::Pin::into_input).
    pub fn floating(self) -> Result<Input<'a, PIN, I2C, Floating, D>, ModeChange<Error<E>, Self>> {
        match self.interface.set_pull_up::<PIN>(false) {
            Ok(()) => Ok(Input {
                interface: self.interface,
                _state: PhantomData,
                _debounce: PhantomData,
            }),
            Err(error) => Err(ModeChange { error, pin: self }),
        }
    }

    /// Configure the pin as a pull-down input.
    ///
    /// # Errors
    /// See [`Pin::into_input`](crate::Pin::into_input).
    pub fn pulldown(self) -> Result<Input<'a, PIN, I2C, PullDown, D>, ModeChange<Error<E>, Self>> {
        let result = (|| -> Result<(), Error<E>> {
            self.interface.set_pull_up::<PIN>(false)?;
            self.interface.set_pull_down::<PIN>(true)?;
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

impl<'a, const PIN: u8, I2C, E, D> Input<'a, PIN, I2C, PullDown, D>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
{
    /// Configure the pin as a floating input.
    ///
    /// # Errors
    /// See [`Pin::into_input`](crate::Pin::into_input).
    pub fn floating(self) -> Result<Input<'a, PIN, I2C, Floating, D>, ModeChange<Error<E>, Self>> {
        match self.interface.set_pull_down::<PIN>(false) {
            Ok(()) => Ok(Input {
                interface: self.interface,
                _state: PhantomData,
                _debounce: PhantomData,
            }),
            Err(error) => Err(ModeChange { error, pin: self }),
        }
    }

    /// Configure the pin as a pull-up input.
    ///
    /// # Errors
    /// See [`Pin::into_input`](crate::Pin::into_input).
    pub fn pullup(self) -> Result<Input<'a, PIN, I2C, PullUp, D>, ModeChange<Error<E>, Self>> {
        let result = (|| -> Result<(), Error<E>> {
            self.interface.set_pull_down::<PIN>(false)?;
            self.interface.set_pull_up::<PIN>(true)?;
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

impl<'a, const PIN: u8, I2C, E, S> Input<'a, PIN, I2C, S, DebounceOff>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
{
    /// Enable debounce for the pin.
    ///
    /// # Errors
    /// See [`Pin::into_input`](crate::Pin::into_input).
    pub fn debounce_on(
        self,
    ) -> Result<Input<'a, PIN, I2C, S, DebounceOn>, ModeChange<Error<E>, Self>> {
        match self.interface.set_debounce_enable::<PIN>(true) {
            Ok(()) => Ok(Input {
                interface: self.interface,
                _state: PhantomData,
                _debounce: PhantomData,
            }),
            Err(error) => Err(ModeChange { error, pin: self }),
        }
    }
}

impl<'a, const PIN: u8, I2C, E, S> Input<'a, PIN, I2C, S, DebounceOn>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
{
    /// Disable debounce for the pin.
    ///
    /// # Errors
    /// See [`Pin::into_input`](crate::Pin::into_input).
    pub fn debounce_off(
        self,
    ) -> Result<Input<'a, PIN, I2C, S, DebounceOff>, ModeChange<Error<E>, Self>> {
        match self.interface.set_debounce_enable::<PIN>(false) {
            Ok(()) => Ok(Input {
                interface: self.interface,
                _state: PhantomData,
                _debounce: PhantomData,
            }),
            Err(error) => Err(ModeChange { error, pin: self }),
        }
    }
}

impl<'a, const PIN: u8, I2C, E> Output<'a, PIN, I2C, PushPull>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
{
    /// Configure the pin as an open-drain output.
    ///
    /// # Errors
    /// See [`Pin::into_output`](crate::Pin::into_output).
    pub fn open_drain(self) -> Result<Output<'a, PIN, I2C, OpenDrain>, ModeChange<Error<E>, Self>> {
        match self.interface.set_open_drain::<PIN>(true) {
            Ok(()) => Ok(Output {
                interface: self.interface,
                _state: PhantomData,
            }),
            Err(error) => Err(ModeChange { error, pin: self }),
        }
    }
}

impl<'a, const PIN: u8, I2C, E> Output<'a, PIN, I2C, OpenDrain>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
{
    /// Configure the pin as a push-pull output.
    ///
    /// # Errors
    /// See [`Pin::into_output`](crate::Pin::into_output).
    pub fn push_pull(self) -> Result<Output<'a, PIN, I2C, PushPull>, ModeChange<Error<E>, Self>> {
        match self.interface.set_open_drain::<PIN>(false) {
            Ok(()) => Ok(Output {
                interface: self.interface,
                _state: PhantomData,
            }),
            Err(error) => Err(ModeChange { error, pin: self }),
        }
    }
}
