use wiringx::{Platform, Polarity, WiringX};

use std::time::{Duration, Instant};

fn main() {
    let wiringx = WiringX::new(Platform::MilkVDuoS).unwrap();

    let mut pwm = wiringx
        .pwm_pin(
            11,                         // pin number
            Duration::from_nanos(1000), // period
            0.0,                        // duty cycle
            Polarity::Normal,
        )
        .unwrap();

    let duty_timestamp = Instant::now();
    loop {
        let duty = ((duty_timestamp.elapsed().as_secs_f32() * 2.0).sin() + 1.0) * 0.5;

        pwm.set_duty_cycle(duty).unwrap()
    }
}
