# blink-atmega169

This is the most simple example embedded application for **ATmega169** written with [Rust](https://www.rust-lang.org).

## Requirements

It is used the __API__ from crate [avr-device](https://github.com/Rahix/avr-device). At the moment of writing there is *no support* of **ATmega169** devices. The support is available with the repository [kolbma/avr-device](https://github.com/kolbma/avr-device).

## Compiling

Run the provided `./cargo.sh build --release`.

Convert the `elf` file to an `ihex` file with:  
`avr-objcopy -S -j .text -j .data -O ihex target/avr-unknown-gnu-atmega169/release/blink-atmega169.elf target/avr-unknown-gnu-atmega169/release/blink-atmega169.hex`

## Flash your device

Flash your **ATmega169** with an **avrdude** command like:  
`avrdude -p m169 -c avrispmkII -P /dev/ttyUSB0 -U target/avr-unknown-gnu-atmega169/release/blink-atmega169.hex`

## Power on your device

Connect an LED with a restistor to **VCC** and **PORTB PB7**.  
Power on your device and see the **LED blinking**.
