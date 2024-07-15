/// An error that occurs when communicating with the SX1509.
#[derive(Debug)]
pub enum Error<EI2C> {
    /// An error occurred on the I2C bus.
    Io(EI2C),
    /// The I2C bus is busy, ie used by another pin at the same time.
    BusBusy,
}

impl<EI2C> embedded_hal::digital::Error for Error<EI2C>
where
    EI2C: core::fmt::Debug,
{
    fn kind(&self) -> embedded_hal::digital::ErrorKind {
        embedded_hal::digital::ErrorKind::Other
    }
}

/// An error that occurs when changing the mode of a pin.
pub struct ModeChange<E, P> {
    /// The inner error that occurred, preventing the mode change.
    pub error: E,
    /// The pin that failed to change mode.
    pub pin: P,
}

impl<E: core::fmt::Debug, P> core::fmt::Debug for ModeChange<E, P> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ModeChangeError")
            .field("error", &self.error)
            .finish_non_exhaustive()
    }
}
