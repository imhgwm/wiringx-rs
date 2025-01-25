//! General purpose input output related objects.

use std::{collections::HashSet, sync::Arc, time::Duration};

use parking_lot::Mutex;
use wiringx_sys::{
    digitalRead, digitalWrite, digital_value_t_HIGH, digital_value_t_LOW, waitForInterrupt,
    wiringXISR,
};

use crate::WiringXError;

/// Representation of a GPIO pin.
#[derive(Debug)]
pub struct Pin<T: Default> {
    number: i32,
    handle: Arc<Mutex<HashSet<i32>>>,
    mode: T,
}

impl<T: Default> Pin<T> {
    pub(super) fn new(number: i32, handle: Arc<Mutex<HashSet<i32>>>) -> Self {
        Self {
            number,
            handle,
            mode: T::default(),
        }
    }

    /// Returns the number of this pin.
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

    /// Returns the current value of this GPIO pin.
    pub fn read(&self) -> Value {
        self.mode.value
    }
}

impl Pin<Input> {
    /// Reads the state of the GPIO pin.
    pub fn read(&self) -> Value {
        let result = unsafe { digitalRead(self.number) };

        if result == 1 {
            Value::High
        } else {
            Value::Low
        }
    }

    /// Sets the interrupt service routine mode of this pin, when to trigger using the `wait_for_interrupt` method.
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
    High,
    #[default]
    Low,
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
