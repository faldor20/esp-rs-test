use embedded_hal::{digital::v2::OutputPin, prelude::_embedded_hal_blocking_delay_DelayMs};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use embedded_graphics::{
    mono_font::{
        ascii::{FONT_10X20, FONT_5X8, FONT_6X12, FONT_9X15},
        MonoTextStyle, MonoTextStyleBuilder,
    },
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};
use esp_idf_hal::{
    delay, gpio,
    i2c::{self, I2c},
    peripherals,
    prelude::*,
};
//use sh1106::{interface::DisplayInterface, mode::graphics, prelude::*, Builder, displayrotation};
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();
    println!("Hello, worldpotato!");
    let periphs = peripherals::Peripherals::take().unwrap();
    let pins = periphs.pins;
    setup_i2c_display(periphs.i2c0, pins.gpio21, pins.gpio22).unwrap();
}

fn setup_i2c_display(
    i2c: i2c::I2C0,
    sda: gpio::Gpio21<gpio::Unknown>,
    scl: gpio::Gpio22<gpio::Unknown>,
) -> Result<(), anyhow::Error> {
    let config = <i2c::config::MasterConfig as Default>::default().baudrate((400 * 1000).into());
    let di = i2c::Master::<i2c::I2C0, _, _>::new(i2c, i2c::MasterPins { sda, scl }, config)?;

    let mut delay = delay::Ets;

    //    let display = sh1106::Builder::new().connect_i2c(di);
    let interface = I2CDisplayInterface::new(di);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();
    let mut graphics = display;
    Text::new(
        "Hello World! - default style 5x8",
        Point::new(15, 15),
        MonoTextStyle::new(&FONT_5X8, BinaryColor::On),
    )
    .draw(&mut graphics)
    .unwrap();

    graphics.set_pixel(5, 5, true);
    graphics
        .flush()
        .map_err(|e| anyhow::anyhow!("Display error: {:?}", e))?;
    println!("done");
    Ok(())
}

// fn drawText<G>(display: &mut GraphicsMode<G>) -> Result<(), anyhow::Error>
// where
//     G: DisplayInterface,
// {
//     // Show smallest font with black font on white background (default value for fonts)
//     Text::new(
//         "Hello World! - default style 5x8",
//         Point::new(15, 15),
//         MonoTextStyle::new(&FONT_5X8, BinaryColor::On),
//     )
//     .draw(&mut display)?;

//     // Show smallest font with white font on black background
//     let style = MonoTextStyleBuilder::new()
//         .font(&FONT_5X8)
//         .text_color(BinaryColor::Off)
//         .background_color(BinaryColor::On)
//         .build();

//     Text::new("Hello World! - inverse 5x8", Point::new(15, 30), style).draw(&mut display)?;

//     // Show 6x12 Font
//     Text::new(
//         "Hello 6x12!",
//         Point::new(15, 45),
//         MonoTextStyle::new(&FONT_6X12, BinaryColor::On),
//     )
//     .draw(&mut display)?;

//     // Show 9x15 Font
//     Text::new(
//         "Hello 9x15!",
//         Point::new(15, 70),
//         MonoTextStyle::new(&FONT_9X15, BinaryColor::On),
//     )
//     .draw(&mut display)?;

//     // Show 10x20 Font
//     Text::new(
//         "Hello 10x20!",
//         Point::new(15, 95),
//         MonoTextStyle::new(&FONT_10X20, BinaryColor::On),
//     )
//     .draw(&mut display)?;

//     // let output_settings = OutputSettingsBuilder::new().scale(2).build();
//     // Window::new("Fonts", &output_settings).show_static(&display);

//     Ok(())
// }
