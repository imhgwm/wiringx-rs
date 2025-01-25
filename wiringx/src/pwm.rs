//! Pulse width modulation related objects.

use std::time::Duration;

use wiringx_sys::{
    wiringXPWMEnable, wiringXPWMSetDuty, wiringXPWMSetPeriod, wiringXPWMSetPolarity,
};

use crate::{Hand, WiringXError};

/// Instance of a pulse-width modulated pin.
#[derive(Debug)]
pub struct PwmPin {
    number: i32,
    handles: Hand<i32>,

    period: Duration,
    duty_cycle: f32,
    polarity: Polarity,
}

impl PwmPin {
    pub(super) fn new(
        number: i32,
        handles: Hand<i32>,
        period: Duration,
        duty_cycle: f32,
        polarity: Polarity,
    ) -> Result<Self, WiringXError> {
        if handles.lock().contains(&number) {
            return Err(WiringXError::PinUsed);
        }

        let result = unsafe { wiringXPWMSetPeriod(number, period.as_nanos() as i64) };

        if result < 0 {
            let result = unsafe { wiringXPWMSetDuty(number, 0) };
            if result < 0 {
                return Err(WiringXError::Unsupported);
            }

            let result = unsafe { wiringXPWMSetPeriod(number, period.as_nanos() as i64) };
            if result < 0 {
                return Err(WiringXError::InvalidArgument);
            }
        }

        let duty_cycle = duty_cycle.clamp(0.0, 1.0);

        let result =
            unsafe { wiringXPWMSetDuty(number, period.mul_f32(duty_cycle).as_nanos() as i64) };

        if result < 0 {
            return Err(WiringXError::InvalidArgument);
        }

        let result = unsafe { wiringXPWMSetPolarity(number, polarity as i32) };

        if result < 0 {
            return Err(WiringXError::InvalidArgument);
        }

        let result = unsafe { wiringXPWMEnable(number, 1) };

        if result < 0 {
            return Err(WiringXError::InvalidArgument);
        }

        handles.lock().insert(number);

        Ok(Self {
            number,
            handles,
            period,
            duty_cycle,
            polarity,
        })
    }

    /// Sets the period of time a PWM cycle takes.
    pub fn set_period(&mut self, period: Duration) -> Result<(), WiringXError> {
        let result = unsafe { wiringXPWMSetPeriod(self.number, period.as_nanos() as i64) };

        if result < 0 {
            return Err(WiringXError::InvalidArgument);
        }

        self.period = period;

        Ok(())
    }

    /// Returns the period duration of this pin.
    pub fn period(&self) -> Duration {
        self.period
    }

    /// Sets the duty cycle of the pin.
    ///
    /// The duty cycle is the proportion of the period the signal is high.
    ///
    /// Takes a value from 0.0 - 1.0, where 0 represents 0% and 1 represents 100%
    ///
    /// Automatically clamps to a value in range, in case the given value is smaller than 0 or bigger than 1.
    pub fn set_duty_cycle(&mut self, duty_cycle: f32) -> Result<(), WiringXError> {
        let duty_cycle = duty_cycle.clamp(0.0, 1.0);

        let result = unsafe {
            wiringXPWMSetDuty(
                self.number,
                self.period.mul_f32(duty_cycle).as_nanos() as i64,
            )
        };

        if result < 0 {
            return Err(WiringXError::InvalidArgument);
        }

        self.duty_cycle = duty_cycle;

        Ok(())
    }

    /// Returns the duty cycle of this pin.
    pub fn duty_cycle(&self) -> f32 {
        self.duty_cycle
    }

    /// Sets the polarity of the PWM pin.
    pub fn set_polarity(&mut self, polarity: Polarity) -> Result<(), WiringXError> {
        let result = unsafe { wiringXPWMSetPolarity(self.number, polarity as i32) };

        if result < 0 {
            return Err(WiringXError::InvalidArgument);
        }

        self.polarity = polarity;

        Ok(())
    }

    /// Returns the polarity of this pin.
    pub fn polarity(&self) -> Polarity {
        self.polarity
    }
}

impl Drop for PwmPin {
    fn drop(&mut self) {
        self.handles.lock().remove(&self.number);
        unsafe { wiringXPWMEnable(self.number, 0) };
    }
}

/// PWM polarity of a pin.
#[derive(Debug, Clone, Copy)]
#[repr(i32)]
pub enum Polarity {
    Normal = 0,
    Inversed = 1,
}
