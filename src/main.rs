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
use sh1106::{prelude::*, Builder};


fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();
    esp_idf_sys::
    println!("Hello, worldpotato!");

drawText()
}

fn setup_i2c_display(){
     let dp = ::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

 let i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 100.khz().into(),
            duty_cycle: DutyCycle::Ratio2to1,
        },
        clocks,
        &mut rcc.apb1,
        1000,
        10,
        1000,
        1000,
    );
let mut display: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();

display.init().unwrap();
display.flush().unwrap();

display.set_pixel(10, 20, 1);

display.flush().unwrap();   
}


fn drawText() { 
    let mut display: <BinaryColor> = SimulatorDisplay::new(Size::new(350, 200));

    // Show smallest font with black font on white background (default value for fonts)
    Text::new(
        "Hello World! - default style 5x8",
        Point::new(15, 15),
        MonoTextStyle::new(&FONT_5X8, BinaryColor::On),
    )
    .draw(&mut display)?;

    // Show smallest font with white font on black background
    let style = MonoTextStyleBuilder::new()
        .font(&FONT_5X8)
        .text_color(BinaryColor::Off)
        .background_color(BinaryColor::On)
        .build();

    Text::new("Hello World! - inverse 5x8", Point::new(15, 30), style).draw(&mut display)?;

    // Show 6x12 Font
    Text::new(
        "Hello 6x12!",
        Point::new(15, 45),
        MonoTextStyle::new(&FONT_6X12, BinaryColor::On),
    )
    .draw(&mut display)?;

    // Show 9x15 Font
    Text::new(
        "Hello 9x15!",
        Point::new(15, 70),
        MonoTextStyle::new(&FONT_9X15, BinaryColor::On),
    )
    .draw(&mut display)?;

    // Show 10x20 Font
    Text::new(
        "Hello 10x20!",
        Point::new(15, 95),
        MonoTextStyle::new(&FONT_10X20, BinaryColor::On),
    )
    .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    Window::new("Fonts", &output_settings).show_static(&display);

    Ok(())
}