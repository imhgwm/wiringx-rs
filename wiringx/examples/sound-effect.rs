use hound;
use std::io::Cursor;
// Add this crate to your `Cargo.toml`
use std::time::Duration;
use wiringx::{Platform, Polarity, WiringX};

fn main() {
    // Initialize WiringX for the PWM pin
    let wiringx = WiringX::new(Platform::MilkVDuoS).unwrap();

    let mut pwm = wiringx
        .pwm_pin(
            11,                         // pin number
            Duration::from_nanos(3000), // period (adjust as needed for your servo or speaker)
            0.0,                        // initial duty cycle
            Polarity::Normal,
        )
        .unwrap();

    let reader = Cursor::new(include_bytes!("💢.wav"));

    let wav_reader = hound::WavReader::new(reader).expect("Failed to open WAV file");
    let spec = wav_reader.spec();

    // Ensure the WAV file is in a format we can handle
    if spec.channels != 1 || spec.sample_rate < 8000 || spec.bits_per_sample != 16 {
        panic!("Unsupported WAV format: must be mono, 16-bit, and at least 8kHz sample rate");
    }

    // Stream through the WAV samples and convert to PWM duty cycles
    for sample in wav_reader.into_samples::<i16>() {
        match sample {
            Ok(value) => {
                // Normalize 16-bit sample to 0.0 - 1.0 for PWM
                let pwm_duty_cycle = (value as f32 + 32768.0) / 65535.0;

                // Set the PWM duty cycle
                pwm.set_duty_cycle(pwm_duty_cycle).unwrap();
            }
            Err(e) => {
                eprintln!("Error reading WAV sample: {}", e);
                break;
            }
        }
    }

    println!("Playback finished.");
}
