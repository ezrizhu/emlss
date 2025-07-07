use embassy_executor::main;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer, Delay};
use esp_idf_hal::gpio::AnyIOPin;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;
use esp_idf_hal::uart;

mod led;
use led::RGB8;
use led::WS2812RMT;

use sds011::SDS011;

//use esp32_nimble::{BLEDevice, BLEScan};

#[main]
async fn main(_s: Spawner) {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    //led
    let mut led = WS2812RMT::new(pins.gpio8, peripherals.rmt.channel0).unwrap();
    led.set_pixel(RGB8::new(128, 0, 128)).unwrap();

    // sds011
    let mut uart_event_config = uart::config::EventConfig::default();
    uart_event_config.rx_fifo_full = Some(10);

    let mut uart_config = uart::config::Config::default().baudrate(Hertz(9_600));
    uart_config.event_config = uart_event_config;

    let mut uart = uart::AsyncUartDriver::new(
        peripherals.uart1,
        pins.gpio1,
        pins.gpio3,
        Option::<AnyIOPin>::None,
        Option::<AnyIOPin>::None,
        &uart_config,
    )
    .unwrap();

    let sds011 = SDS011::new(&mut uart, sds011::Config::default());
    let mut sds011 = sds011.init(&mut Delay).await.unwrap();

    log::info!("measuring");
    let dust = sds011.measure(&mut Delay).await.unwrap();
    log::info!("{}", dust);
    loop {
        Timer::after(Duration::from_secs(10)).await;
        log::info!("10s");
    }

    //log::info!("ble");
    //let ble_device = BLEDevice::take();
    //let mut ble_scan = BLEScan::new();
    //let ble_device = BLEDevice::take();
    //let mut ble_scan = BLEScan::new();
    //ble_scan.active_scan(true).interval(100).window(99);

    //ble_scan
    //    .start(ble_device, 5000, |device, data| {
    //        log::info!("Advertised Device: ({:?}, {:?})", device, data);
    //        None::<()>
    //    })
    //    .await.unwrap();

    //log::info!("Scan end");
}
