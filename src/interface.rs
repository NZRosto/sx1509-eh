use crate::{error::Error, reg::Register};

#[derive(Clone, Copy)]
enum BankAgnosticRegister {
    Dir,
    Data,
    PullUp,
    PullDown,
    OpenDrain,
    DebounceEnable,
}

impl BankAgnosticRegister {
    pub(crate) const fn into_register<const PIN: u8>(self) -> Register {
        if const { PIN < 8 } {
            match self {
                BankAgnosticRegister::Dir => Register::RegDirA,
                BankAgnosticRegister::Data => Register::RegDataA,
                BankAgnosticRegister::PullUp => Register::RegPullUpA,
                BankAgnosticRegister::PullDown => Register::RegPullDownA,
                BankAgnosticRegister::OpenDrain => Register::RegOpenDrainA,
                BankAgnosticRegister::DebounceEnable => Register::RegDebounceEnableA,
            }
        } else {
            match self {
                BankAgnosticRegister::Dir => Register::RegDirB,
                BankAgnosticRegister::Data => Register::RegDataB,
                BankAgnosticRegister::PullUp => Register::RegPullUpB,
                BankAgnosticRegister::PullDown => Register::RegPullDownB,
                BankAgnosticRegister::OpenDrain => Register::RegOpenDrainB,
                BankAgnosticRegister::DebounceEnable => Register::RegDebounceEnableB,
            }
        }
    }
}

/// Debounce time, if enabled for a certain pin.
#[derive(Debug, Default, Clone, Copy)]
pub enum DebounceTime {
    /// 0.5ms
    #[default]
    Ms0_5 = 0b000,
    /// 1ms
    Ms1 = 0b001,
    /// 2ms
    Ms2 = 0b010,
    /// 4ms
    Ms4 = 0b011,
    /// 8ms
    Ms8 = 0b100,
    /// 16ms
    Ms16 = 0b101,
    /// 32ms
    Ms32 = 0b110,
    /// 64ms
    Ms64 = 0b111,
}

pub(crate) struct Interface<I2C> {
    i2c: spin::Mutex<I2C>,
    address: u8,
}

impl<I2C, E> Interface<I2C>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
{
    pub(crate) fn new(i2c: spin::Mutex<I2C>, address: u8) -> Self {
        Self { i2c, address }
    }

    pub(crate) fn set_output<const PIN: u8>(&self) -> Result<(), Error<E>> {
        self.set_bit::<PIN>(BankAgnosticRegister::Dir)
    }

    pub(crate) fn set_input<const PIN: u8>(&self) -> Result<(), Error<E>> {
        self.unset_bit::<PIN>(BankAgnosticRegister::Dir)
    }

    pub(crate) fn set_data<const PIN: u8>(&self, value: bool) -> Result<(), Error<E>> {
        if value {
            self.set_bit::<PIN>(BankAgnosticRegister::Data)
        } else {
            self.unset_bit::<PIN>(BankAgnosticRegister::Data)
        }
    }

    pub(crate) fn get_data<const PIN: u8>(&self) -> Result<bool, Error<E>> {
        self.get_bit::<PIN>(BankAgnosticRegister::Data)
    }

    pub(crate) fn set_pull_up<const PIN: u8>(&self, value: bool) -> Result<(), Error<E>> {
        if value {
            self.set_bit::<PIN>(BankAgnosticRegister::PullUp)
        } else {
            self.unset_bit::<PIN>(BankAgnosticRegister::PullUp)
        }
    }

    pub(crate) fn set_pull_down<const PIN: u8>(&self, value: bool) -> Result<(), Error<E>> {
        if value {
            self.set_bit::<PIN>(BankAgnosticRegister::PullDown)
        } else {
            self.unset_bit::<PIN>(BankAgnosticRegister::PullDown)
        }
    }

    pub(crate) fn set_open_drain<const PIN: u8>(&self, value: bool) -> Result<(), Error<E>> {
        if value {
            self.set_bit::<PIN>(BankAgnosticRegister::OpenDrain)
        } else {
            self.unset_bit::<PIN>(BankAgnosticRegister::OpenDrain)
        }
    }

    pub(crate) fn set_debounce_enable<const PIN: u8>(&self, value: bool) -> Result<(), Error<E>> {
        if value {
            self.set_bit::<PIN>(BankAgnosticRegister::DebounceEnable)
        } else {
            self.unset_bit::<PIN>(BankAgnosticRegister::DebounceEnable)
        }
    }

    pub(crate) fn set_debounce_time(&self, debounce_time: DebounceTime) -> Result<(), Error<E>> {
        self.write(Register::RegDebounceConfig, debounce_time as u8)
    }
}

impl<I2C, E> Interface<I2C>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
{
    fn set_bit<const PIN: u8>(&self, bar: BankAgnosticRegister) -> Result<(), Error<E>> {
        let register = bar.into_register::<PIN>();

        if const { PIN < 8 } {
            let existing_data = self.read(register)?;
            let new_data = existing_data | (1 << PIN);
            self.write(register, new_data)
        } else {
            let existing_data = self.read(register)?;
            let new_data = existing_data | (1 << (PIN - 8));
            self.write(register, new_data)
        }
    }

    fn unset_bit<const PIN: u8>(&self, bar: BankAgnosticRegister) -> Result<(), Error<E>> {
        let register = bar.into_register::<PIN>();

        if const { PIN < 8 } {
            let existing_data = self.read(register)?;
            let new_data = existing_data & !(1 << PIN);
            self.write(register, new_data)
        } else {
            let existing_data = self.read(register)?;
            let new_data = existing_data & !(1 << (PIN - 8));
            self.write(register, new_data)
        }
    }

    fn get_bit<const PIN: u8>(&self, bar: BankAgnosticRegister) -> Result<bool, Error<E>> {
        let register = bar.into_register::<PIN>();

        if const { PIN < 8 } {
            let data = self.read(register)?;
            Ok(data & (1 << PIN) != 0)
        } else {
            let data = self.read(register)?;
            Ok(data & (1 << (PIN - 8)) != 0)
        }
    }

    fn write(&self, register: Register, data: u8) -> Result<(), Error<E>> {
        self.i2c
            .try_lock()
            .ok_or(Error::BusBusy)?
            .write(self.address, &[register as u8, data])
            .map_err(Error::Io)?;
        Ok(())
    }

    fn read(&self, register: Register) -> Result<u8, Error<E>> {
        let mut data = [0];
        self.i2c
            .try_lock()
            .ok_or(Error::BusBusy)?
            .write_read(self.address, &[register as u8], &mut data)
            .map_err(Error::Io)?;
        Ok(data[0])
    }
}
