# Simple DHT11

The aim of this library is to create an incredibly quick and easy way for a user to hook up a DHT11 to a Raspberry Pi and get a reading from it.

## Example

```rust
use simple_dht11::dht11::Dht11;

fn main() {
    let mut dht11 = Dht11::new(27); // Note this is BCM

    let response = dht11.get_reading();

    println!("Temperature: {}", response.temperature);
    println!("Humidity: {}", response.humidity);
}
```

Example output:

```
Temperature: 24.9
Humidity: 21
```

> ⚠️ If you are cross compiling, please see the note about this on [the RPPal Repo!](https://github.com/golemparts/rppal#cross-compilation)