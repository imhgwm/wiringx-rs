use wiringx::{Platform, Polarity, WiringX};

use std::{io, time::Duration};

fn main() {
    let wiringx = WiringX::new(Platform::MilkVDuoS).unwrap();

    let mut pwm = wiringx
        .pwm_pin(11, Duration::from_millis(20), 0.0, Polarity::Normal)
        .unwrap();

    let mut buf = String::new();
    loop {
        let _ = io::stdin().read_line(&mut buf);
        pwm.set_duty_cycle(0.075).unwrap();
        let _ = io::stdin().read_line(&mut buf);
        pwm.set_duty_cycle(0.05).unwrap();
        let _ = io::stdin().read_line(&mut buf);
        pwm.set_duty_cycle(0.075).unwrap();
        let _ = io::stdin().read_line(&mut buf);
        pwm.set_duty_cycle(0.1).unwrap();
    }
}
