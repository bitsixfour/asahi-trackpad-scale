use evdev::{Device, EventSummary, AbsoluteAxisCode};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut trackpad = Device::open("/dev/input/event2")?;
    let status: &str = trackpad.name().unwrap_or("I don't know");

    println!("starting function on {}", status);
    println!("vat is the supported inputs: {:?}", trackpad.get_abs_state());
    for ev in trackpad.fetch_events()? {
         if let EventSummary::AbsoluteAxis(_, AbsoluteAxisCode::ABS_PRESSURE, value) =
       ev.destructure() {
             println!("pressure = {value}"); // prints 12 for your example
         }
    }

    Ok(())
}
