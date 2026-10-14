use esp_hal::gpio::Output;

pub struct Led<'d> {
    pin: Output<'d>,
    color: Color
}

impl<'d> Led<'d> {
    pub fn new(pin: Output<'d>, color: Color) -> Self {
        Led { pin, color }
    }
    
    pub fn is_high(&self) -> bool {
        self.pin.is_set_low()
    }

    pub fn on(&mut self) {
        self.pin.set_high();
    }

    pub fn off(&mut self) {
        self.pin.set_low();
    }

    pub fn color(&self) -> Color {
        self.color
    }
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    Red,
    White,
    Blue,
    Yellow,
    Green
}
