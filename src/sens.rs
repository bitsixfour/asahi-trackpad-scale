use evdev::{Device, EventSummary, AbsoluteAxisCode};
pub struct Sens {
    pub scl: i32,
    pub device: evdev::Device,
    pub name:  &str,
}
impl Sens {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut trackpad = Device::open("/dev/input/event2")?;
        let status: &str = trackpad.name()
            .unwrap_or("I don't know");
        Ok( Self {
            scl: 0,
        }
        )
    }
}
