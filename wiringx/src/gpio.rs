//! General purpose input output related objects.

use std::{collections::HashSet, sync::Arc, time::Duration};

use parking_lot::Mutex;
use wiringx_sys::{
    digitalRead, digitalWrite, digital_value_t_HIGH, digital_value_t_LOW, waitForInterrupt,
    wiringXISR,
};

use crate::WiringXError;

/// Representation of a GPIO, General Purpose Input Output, pin.
///
/// You receive this struct from the [`WiringX::gpio_pin`](super::WiringX::gpio_pin)
/// method of the [`WiringX`](super::WiringX) struct.
#[derive(Debug)]
pub struct Pin<T: Default> {
    number: i32,
    handle: Arc<Mutex<HashSet<i32>>>,
    mode: T,
}

impl<T: Default> Pin<T> {
    #[inline]
    pub(super) fn new(number: i32, handle: Arc<Mutex<HashSet<i32>>>) -> Self {
        Self {
            number,
            handle,
            mode: T::default(),
        }
    }

    /// Returns the number of this pin.
    #[inline]
    pub fn number(&self) -> i32 {
        self.number
    }
}

impl Pin<Output> {
    /// Writes a value to the GPIO pin.
    pub fn write(&mut self, value: Value) {
        self.mode.value = value;

        let value = match value {
            Value::High => digital_value_t_HIGH,
            Value::Low => digital_value_t_LOW,
        };

        unsafe { digitalWrite(self.number, value) };
    }

    /// Toggles the GPIO pin to on if it was off or to off if it was on.
    pub fn toggle(&mut self) {
        self.write(self.read().opposite());
    }

    /// Returns the current value of this GPIO pin.
    #[inline]
    pub fn read(&self) -> Value {
        // in PinMode Output return the current output state
        let result = unsafe { digitalRead(self.number) };

        if result == 1 {
            Value::High
        } else {
            Value::Low
        }
    }
}

impl Pin<Input> {
    /// Reads the current state of the GPIO pin.
    pub fn read(&self) -> Value {
        let result = unsafe { digitalRead(self.number) };

        if result == 1 {
            Value::High
        } else {
            Value::Low
        }
    }

    /// Sets the interrupt service routine mode of this pin.
    ///
    /// This determines when to trigger the interrupt when using the `wait_for_interrupt` method.
    pub fn set_isr_mode(&self, mode: IsrMode) -> Result<(), WiringXError> {
        let result = unsafe { wiringXISR(self.number, mode as u32) };

        if result < 0 {
            return Err(WiringXError::Other(
                "Cannot set isr mode of pin to this setting.".to_string(),
            ));
        }

        Ok(())
    }

    /// Suspends the thread until input to this pin was detected or the function times out.
    ///
    /// Returns `Ok(())` on successful interrupt read and `Err(InterruptTimeOut)` on timeout.
    pub fn wait_for_interrupt(&self, timeout_dur: Duration) -> Result<(), InterruptTimeOut> {
        let result = unsafe { waitForInterrupt(self.number, timeout_dur.as_millis() as i32) };

        if result < 1 {
            Err(InterruptTimeOut)
        } else {
            Ok(())
        }
    }
}

impl<T: Default> Drop for Pin<T> {
    fn drop(&mut self) {
        self.handle.lock().remove(&self.number);
    }
}

/// Sets the pin mode to output, allowing writing to the pin value.
#[derive(Debug, Clone, Copy, Default)]
pub struct Output {
     value: Value,
}

/// Sets the pin mode to input, allowing reading the physical value.
#[derive(Debug, Clone, Copy, Default)]
pub struct Input;

/// Digital voltage value of the pin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Value {
    /// Low current or "off"
    #[default]
    Low = 0,
    /// High current or "on"
    High = 1,
}

impl Value {
    /// Returns the opposite value, returning [`Low`](Value::Low) when [`High`](Value::High)
    /// and vice-versa.
    pub fn opposite(&self) -> Self {
        match self {
            Self::Low => Self::High,
            Self::High => Self::Low,
        }
    }
}

/// Returned if a interrupt function times out.
#[derive(Debug, Clone, Copy)]
pub struct InterruptTimeOut;

/// Mode for the interrupt service routine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IsrMode {
    Unknown = 0,
    Rising = 2,
    Falling = 4,
    Both = 8,
    None = 16,
}
