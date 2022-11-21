use rppal::gpio::{Gpio, IoPin, Mode};
use std::thread::sleep;
use std::time::Duration;

pub struct Dht11 {
    // The pin the DHT11 is connected to.
    pin_number: u8,

    // The IoPin used for communication with the DHT11
    pin: Option<IoPin>,
}

pub struct Dht11Reading {
    // The temperature in degrees Celsius
    pub temperature: f32,

    // The humidity in percent
    pub humidity: f32,
}

impl Dht11 {
    // Create a new DHT11 instance.
    pub fn new(pin_number: u8) -> Dht11 {
        let mut dht11 = Dht11 {
            pin_number,
            pin: None,
        };

        dht11.init_pin();

        dht11
    }

    // Attempt to get a reading from the DHT11.
    pub fn get_reading(&mut self) -> Dht11Reading {
        let mut reading = self.read_data();

        while reading.is_none() {
            // Sleep for 500ms and try again
            sleep(Duration::from_millis(500));

            reading = self.read_data();
        }

        reading.unwrap()
    }

    // Initialize the pin
    fn init_pin(&mut self) {
        // Initialize the pin.
        self.pin = Some(
            Gpio::new()
                .unwrap()
                .get(self.pin_number)
                .unwrap()
                .into_io(Mode::Output),
        );
    }

    // Read the temperature and humidity from the DHT11.
    fn read_data(&mut self) -> Option<Dht11Reading> {
        // The data bytes length received from the DHT11.
        const DATA_LENGTH: u8 = 40;
        let mut data: [u8; DATA_LENGTH as usize] = [0; DATA_LENGTH as usize];
        let mut has_received_response = false;
        let mut data_response_counter: u8 = 0;

        self.send_start_signal();

        self.pin.as_mut().unwrap().set_mode(Mode::Input);

        if let Some(pin) = &self.pin {
            while data_response_counter < DATA_LENGTH {
                let start_time = std::time::Instant::now();
                while pin.is_low() {
                    // wait for pin to go high
                    // if its > 100ms, then we return
                    if start_time.elapsed().as_millis() > 100 {
                        return None;
                    }
                }

                let set_high_time = std::time::Instant::now();

                while pin.is_high() {
                    // wait for pin to go low
                    // if its > 100ms, then we return
                    if set_high_time.elapsed().as_millis() > 100 {
                        return None;
                    }
                }

                let set_low_time = std::time::Instant::now();

                let duration = set_low_time.duration_since(set_high_time);

                if !has_received_response {
                    has_received_response = true;
                    // We skip the first reading, because it is the response from the DHT11.
                    continue;
                }

                if duration < Duration::from_micros(80) {
                    if duration > Duration::from_micros(30) {
                        data[data_response_counter as usize] = 1;
                    } else {
                        data[data_response_counter as usize] = 0;
                    }

                    data_response_counter += 1;
                }
            }
        }

        let mut humidity_int: u8 = 0;
        let mut humidity_dec: u8 = 0;
        let mut temperature_int: u8 = 0;
        let mut temperature_dec: u8 = 0;

        for i in 0..8 {
            humidity_int += data[i] << (7 - i);
        }

        for i in 0..8 {
            humidity_dec += data[i + 8] << (7 - i);
        }

        for i in 0..8 {
            temperature_int += data[i + 16] << (7 - i);
        }

        for i in 0..8 {
            temperature_dec += data[i + 24] << (7 - i);
        }

        let humidity = humidity_int as f32 + humidity_dec as f32 / 10.0;
        let temperature = temperature_int as f32 + temperature_dec as f32 / 10.0;

        Some(Dht11Reading {
            temperature,
            humidity,
        })
    }

    fn send_start_signal(&mut self) {
        self.pin.as_mut().unwrap().set_mode(Mode::Output);

        // Set the pin to high
        self.pin.as_mut().unwrap().set_high();

        // Wait for 30ms
        sleep(Duration::from_millis(30));

        // Set the pin to low
        self.pin.as_mut().unwrap().set_low();

        // Wait for 20ms
        sleep(Duration::from_millis(20));

        // Set the pin to high
        self.pin.as_mut().unwrap().set_high();

        // Wait for 40us
        sleep(Duration::from_micros(40));
    }
}
