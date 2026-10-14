#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull};
use esp_hal::main;
use esp_hal::time::{Duration, Instant};
use esp_println::println;

use blinky::led::{Color, Led};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

fn delay_ms(ms: u64) {
    let start = Instant::now();
    while start.elapsed() < Duration::from_millis(ms) {}
}

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let mut leds = [
        Led::new(
            Output::new(peripherals.GPIO23, Level::Low, OutputConfig::default()),
            Color::Yellow,
        ),
        Led::new(
            Output::new(peripherals.GPIO22, Level::Low, OutputConfig::default()),
            Color::Blue,
        ),
        Led::new(
            Output::new(peripherals.GPIO21, Level::Low, OutputConfig::default()),
            Color::White,
        ),
        Led::new(
            Output::new(peripherals.GPIO16, Level::Low, OutputConfig::default()),
            Color::Red,
        ),
        Led::new(
            Output::new(peripherals.GPIO2, Level::Low, OutputConfig::default()),
            Color::Green,
        ),
    ];

    let first_button = Input::new(
        peripherals.GPIO15,
        InputConfig::default().with_pull(Pull::Up),
    );

    let second_button = Input::new(
        peripherals.GPIO18,
        InputConfig::default().with_pull(Pull::Up),
    );

    println!("All LEDs are turned off");

    let mut current = 0;
    let mut enabled = false;
    let mut first_btn_state = false;
    let mut second_btn_state = false;

    loop {
        let first_pressed = first_button.is_low();
        let second_pressed = second_button.is_low();

        if first_pressed && !first_btn_state {
            enabled = !enabled;
            if enabled {
                current = 0;
                leds[current].on();
                println!("Turn on LED: {:?}", leds[current].color());
            } else {
                for led in leds.iter_mut() {
                    led.off();
                }
                println!("All LEDs are turned off");
            }
        }

        if enabled && second_pressed && !second_btn_state {
            leds[current].off();
            current = (current + 1) % leds.len();
            leds[current].on();
            println!("Turn on LED: {:?}", leds[current].color());
        }

        first_btn_state = first_pressed;
        second_btn_state = second_pressed;
        delay_ms(20);
    }
}
