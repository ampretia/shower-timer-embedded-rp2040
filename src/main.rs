//! XIAO RP2040 Blinking LED Example
//!
//! https://tutoduino.fr/
//!
//! Blinks the USER LED and NeoPixel RGB LED on a Seeed Studio XIAO RP2040 board.
//!

#![no_std]
#![no_main]

use embedded_hal::digital::v2::OutputPin;
use hal::pio::PIOExt;
use hal::Timer;
use panic_halt as _;
use seeeduino_xiao_rp2040::entry;
use seeeduino_xiao_rp2040::hal;
use seeeduino_xiao_rp2040::hal::pac;
use seeeduino_xiao_rp2040::hal::prelude::*;
use smart_leds::SmartLedsWrite;
use smart_leds::RGB8;
use ws2812_pio::Ws2812;

const LUMINOSITY: u8 = 10;
const RED: RGB8 = RGB8::new(LUMINOSITY, 0, 0);
const GREEN: RGB8 = RGB8::new(0, LUMINOSITY, 0);
const BLUE: RGB8 = RGB8::new(0, 0, LUMINOSITY);
const WHITE: RGB8 = RGB8::new(LUMINOSITY / 3, LUMINOSITY / 3, LUMINOSITY / 3);

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
    let timer = Timer::new(pac.TIMER, &mut pac.RESETS);

    // Setup PIO
    let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);

    // Setup Neopixel RGB LED
    let mut ws = Ws2812::new(
        pins.neopixel_data.into_mode(),
        &mut pio,
        sm0,
        clocks.peripheral_clock.freq(),
        timer.count_down(),
    );

    // Power on Neopixel RGB LED
    let mut neopixel_power = pins.neopixel_power.into_push_pull_output();
    neopixel_power.set_high().unwrap();

    // Configure the USER LED pins to operate as a push-pull output
    let mut led_blue_pin = pins.led_blue.into_push_pull_output();
    let mut led_green_pin = pins.led_green.into_push_pull_output();
    let mut led_red_pin = pins.led_red.into_push_pull_output();

    loop {
        // Set USER LED to blue
        led_blue_pin.set_low().unwrap();
        led_red_pin.set_high().unwrap();
        led_green_pin.set_high().unwrap();
        // Set RGB LED to blue
        ws.write([BLUE].iter().copied()).unwrap();
        delay.delay_ms(500);

        // Set USER LED to red
        led_blue_pin.set_high().unwrap();
        led_red_pin.set_low().unwrap();
        led_green_pin.set_high().unwrap();
        // Set RGB LED to red
        ws.write([RED].iter().copied()).unwrap();
        delay.delay_ms(500);

        // Set USER LED to green
        led_blue_pin.set_high().unwrap();
        led_red_pin.set_high().unwrap();
        led_green_pin.set_low().unwrap();
        // Set RGB LED to green
        ws.write([GREEN].iter().copied()).unwrap();
        delay.delay_ms(500);

        // Set USER LED to white
        led_blue_pin.set_low().unwrap();
        led_red_pin.set_low().unwrap();
        led_green_pin.set_low().unwrap();
        // Set RGB LED to white
        ws.write([WHITE].iter().copied()).unwrap();
        delay.delay_ms(500);
    }
}
