#![no_std]
#![no_main]

extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
extern crate sam3x8e;

use cortex_m_rt::entry;
use sam3x8e::RTT;

fn delay_ms(rtt: &RTT, ms: u32) {
    // We're not considering overflow here, 32 bits can keep about 49 days in ms
    let now = rtt.vr.read().bits();
    let until = now + ms;

    while rtt.vr.read().bits() < until {}
}

#[entry]
fn main() -> ! {
    let p = sam3x8e::Peripherals::take().unwrap();

    // Enable PIOB
    let pmc = p.PMC;
    pmc.pmc_pcer0.write_with_zero(|w| w.pid12().set_bit());

    // Configure RTT resolution to approx. 1ms
    let rtt = p.RTT;
    rtt.mr.write_with_zero(|w| unsafe { w.rtpres().bits(0x20) });

    let piob = p.PIOB;

    // Configure PIOB pin 27 (LED)
    piob.per.write_with_zero(|w| w.p27().set_bit());
    piob.oer.write_with_zero(|w| w.p27().set_bit());
    piob.pudr.write_with_zero(|w| w.p27().set_bit());

    // On/off blinking
    loop {
        piob.sodr.write_with_zero(|w| w.p27().set_bit());
        delay_ms(&rtt, 1000);
        piob.codr.write_with_zero(|w| w.p27().set_bit());
        delay_ms(&rtt, 1000);
    }
}
