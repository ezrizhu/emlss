use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::hal::task::block_on;
mod led;
use led::RGB8;
use led::WS2812RMT;

//use esp32_nimble::{BLEDevice, BLEScan};

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let peripherals = Peripherals::take()?;
    let mut led = WS2812RMT::new(peripherals.pins.gpio8, peripherals.rmt.channel0)?;
    led.set_pixel(RGB8::new(128, 0, 128))?;

    Ok(())

    //let ble_device = BLEDevice::take();
    //let mut ble_scan = BLEScan::new();
    //block_on(async {
    //    let ble_device = BLEDevice::take();
    //    let mut ble_scan = BLEScan::new();
    //    ble_scan.active_scan(true).interval(100).window(99);

    //    ble_scan
    //        .start(ble_device, 5000, |device, data| {
    //            log::info!("Advertised Device: ({:?}, {:?})", device, data);
    //            None::<()>
    //        })
    //        .await?;

    //    log::info!("Scan end");

    //    Ok(())
    //})
}
