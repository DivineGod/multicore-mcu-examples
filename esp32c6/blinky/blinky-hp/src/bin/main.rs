//! This shows a very basic example of running code on the LP core.
//!
//! Code on LP core increments a counter and continuously toggles LED. The
//! current value is printed by the HP core.
//!
//! Make sure to first compile the `esp-lp-hal/examples/blinky.rs` example
//!
//! The following wiring is assumed:
//! - LED => GPIO1

//% CHIPS: esp32c6
//% FEATURES: esp-hal/unstable

#![no_std]
#![no_main]

use defmt_rtt as _;
use esp_backtrace as _; // global logger
use esp_hal::{
    delay::Delay,
    gpio::lp_io::LowPowerOutput,
    load_lp_code,
    lp_core::{LpCore, LpCoreWakeupSource},
    main,
};

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let delay = Delay::new();

    defmt::trace!("trace");
    defmt::debug!("debug");
    defmt::info!("info");
    defmt::warn!("warn");
    defmt::error!("horse");
    // configure GPIO 1 as LP output pin

    let lp_pin = LowPowerOutput::new(peripherals.GPIO1);

    let mut lp_core = LpCore::new(peripherals.LP_CORE);
    lp_core.stop();
    defmt::info!("lp core stopped");

    // load code to LP core
    // The path is from the crate or workspace root
    let lp_core_code =
        load_lp_code!("../blinky-lp/target/riscv32imac-unknown-none-elf/release/blinky-lp");

    // start LP core
    lp_core_code.run(&mut lp_core, LpCoreWakeupSource::HpCpu, lp_pin);
    defmt::info!("lpcore run");

    let data = (0x5000_2000) as *mut u32;
    loop {
        delay.delay_millis(500u32);
        defmt::println!("Current {:x}           \u{000d}", unsafe {
            data.read_volatile()
        });
    }
}
