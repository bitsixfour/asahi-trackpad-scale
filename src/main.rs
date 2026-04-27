use evdev::{Device, EventSummary, AbsoluteAxisCode};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

mod sens;
use sens::sens;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut trackpad = Device::open("/dev/input/event2")?;
    let status: &str = trackpad.name().unwrap_or("I don't know");

    println!("starting function on {}", status);

    loop {
        for ev in trackpad.fetch_events()? {
            if let EventSummary::AbsoluteAxis(_, AbsoluteAxisCode::ABS_MT_PRESSURE, value) =
            ev.destructure() {
                println!("pressure = {value}"); // prints 12 for your example
            }
        }
    }
    Ok(())
}
