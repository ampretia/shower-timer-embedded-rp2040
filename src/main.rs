//! XIAO RP2040 Shower timer

#![no_std]
#![no_main]
use embedded_hal::digital::v2::OutputPin;// use hal::pio::PIOExt;
// use hal::Timer;
use panic_halt as _;
use seeeduino_xiao_rp2040::entry;
use seeeduino_xiao_rp2040::hal;
use seeeduino_xiao_rp2040::hal::pac;
use seeeduino_xiao_rp2040::hal::prelude::*;

const PHASE_TIME: u32 = 4 * 60 * 1000;

/// Entry point to our bare-metal application.
///
/// The `#[entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables are initialised.
///
/// The function configures the RP2040 peripherals, then blinks the LED in an
/// infinite loop.
#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock from 12 Mhz crystal
    let clocks = hal::clocks::init_clocks_and_plls(
        seeeduino_xiao_rp2040::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = seeeduino_xiao_rp2040::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // The delay object lets us wait for specified amounts of time (in
    // milliseconds)
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    // let timer = Timer::new(pac.TIMER, &mut pac.RESETS);

    // Setup PIO
    // let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);

    // inbuilt led
    let mut built_b_led = pins.led_blue.into_push_pull_output();
    built_b_led.set_high().unwrap();

    let mut built_g_led = pins.led_green.into_push_pull_output();
    built_g_led.set_high().unwrap();

    let mut built_r_led = pins.led_red.into_push_pull_output();
    built_r_led.set_high().unwrap();


    // --------------------------------------

    // Configure the USER LED pins to operate as a push-pull output
    let mut led_blue_pin = pins.scl.into_push_pull_output();
    let mut led_red_pin = pins.sda.into_push_pull_output();
    let mut led_green_pin = pins.tx.into_push_pull_output();

    // Set USER LED to blue
    led_blue_pin.set_low().unwrap();
    led_red_pin.set_high().unwrap();
    led_green_pin.set_low().unwrap();

    delay.delay_ms(1000);

    // Set USER LED to red
    led_blue_pin.set_high().unwrap();
    led_red_pin.set_low().unwrap();
    led_green_pin.set_low().unwrap();
    delay.delay_ms(1000);

    // Set USER LED to blue
    led_blue_pin.set_low().unwrap();
    led_red_pin.set_high().unwrap();
    led_green_pin.set_low().unwrap();

    delay.delay_ms(1000);

    // Set USER LED to green
    led_blue_pin.set_low().unwrap();
    led_red_pin.set_low().unwrap();
    led_green_pin.set_high().unwrap();
    // Set RGB LED to green
    delay.delay_ms(1000);

    led_blue_pin.set_high().unwrap();
    led_red_pin.set_low().unwrap();
    led_green_pin.set_low().unwrap();
    delay.delay_ms(1000);

    // ----------------------------------------
    led_blue_pin.set_low().unwrap();
    led_red_pin.set_low().unwrap();
    led_green_pin.set_high().unwrap();
    delay.delay_ms(PHASE_TIME);

    led_blue_pin.set_high().unwrap();
    led_red_pin.set_low().unwrap();
    led_green_pin.set_low().unwrap();
    delay.delay_ms(PHASE_TIME);

    led_blue_pin.set_low().unwrap();
    led_red_pin.set_high().unwrap();
    led_green_pin.set_low().unwrap();
    delay.delay_ms(PHASE_TIME);

    led_blue_pin.set_low().unwrap();
    led_green_pin.set_low().unwrap();

    loop {
        // Set USER LED to white

        led_red_pin.set_high().unwrap();
        // Set RGB LED to white
        delay.delay_ms(500);
        led_red_pin.set_low().unwrap();
        // Set RGB LED to white
        delay.delay_ms(500);
    }
}
