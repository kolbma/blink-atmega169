#![warn(clippy::all)]
#![no_std]
#![cfg_attr(not(test), no_main)] // #![no_main] interfers with 'cargo test' when targeting the host machine.
#![feature(abi_avr_interrupt)]

extern crate avr_std_stub;

pub use avr_device::atmega169 as avr_dev;

#[no_mangle]
// #[cfg(not(test))] // The main function interfers with 'cargo test' when targeting the host machine.
pub extern "C" fn main() {
    let p = avr_dev::Peripherals::take().unwrap();

    p.PORTB.ddrb.write(|w| unsafe { w.bits(0xFF) });

    loop {
        p.PORTB.portb.write(|w| unsafe { w.bits(0xFF) });
        delay(2000);
        p.PORTB.portb.write(|w| unsafe { w.bits(0x00) });
        delay(2000);
    }
}

fn delay(millisec: u16) {
    let mut m = millisec;
    while 0 < m {
        m -= 1;
        for _i in 0u8..125 {}
    }
}
