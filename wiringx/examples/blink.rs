use wiringx::{Output, Platform, WiringX};

use std::{thread, time::Duration};

fn main() {
    let wiringx = WiringX::new(Platform::MilkVDuoS).unwrap();

    let mut pin = wiringx.gpio_pin::<Output>(0).unwrap();

    loop {
        pin.toggle();
        thread::sleep(Duration::from_secs(1));
    }
}
