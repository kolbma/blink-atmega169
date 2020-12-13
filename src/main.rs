#![warn(clippy::all)]
#![no_std]
#![cfg_attr(not(test), no_main)] // #![no_main] interfers with 'cargo test' when targeting the host machine.
#![feature(llvm_asm)]

extern crate avr_std_stub;

pub use avr_device::atmega169 as avr_dev;

#[no_mangle]
#[cfg(not(test))] // The main function interfers with 'cargo test' when targeting the host machine.
pub extern "C" fn main() {
    let p = avr_dev::Peripherals::take().unwrap();

    p.PORTB.ddrb.write(|w| w.pb7().set_bit());

    loop {
        p.PORTB.portb.write(|w| w.pb7().clear_bit());
        delay(500);
        p.PORTB.portb.write(|w| w.pb7().set_bit());
        delay(350);
    }
}

fn delay(millisec: u16) {
    let mut m = millisec;
    while m > 0 {
        m -= 1;
        for _i in 0u16..1000 {
            unsafe {
                // need to have a nop for sideeffect or it will be optimized away
                llvm_asm!("nop");
            }
        }
    }
}
