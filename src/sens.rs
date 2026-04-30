use evdev::{Device, EventSummary, AbsoluteAxisCode};
use std::{
    env,
    fs,
    path::Path,
    sync::{Arc, Mutex},
    thread,
};
use anyhow::{bail, Context};
const M: f64 = 1000.0;
pub struct Sens {
    pub pressure: Arc<Mutex<i32>>,
    pub name: String,
    pub path: String,
}

impl Sens {
    pub fn new() -> anyhow::Result<Self> {
        let path = pick_event_device()?;
        let mut device = Device::open(&path)
            .with_context(|| format!("failed to open input device at {path}"))?;
        let name = device.name().unwrap_or("unknown").to_string();

        let pressure = Arc::new(Mutex::new(0i32));
        let pressure_clone = Arc::clone(&pressure);

        thread::spawn(move || {
            loop {
                match device.fetch_events() {
                    Ok(events) => {
                        for ev in events {
                            if let EventSummary::AbsoluteAxis(_, AbsoluteAxisCode::ABS_MT_PRESSURE, value,) = ev.destructure()
                            {
                                let mut p = pressure_clone.lock().unwrap();
                                *p = value;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("fetch_events error: {e}");
                        break;
                    }
                }
            }
        });

        Ok(Self { pressure, name, path })
    }

    pub fn get_pressure(&self) -> i32 {
        *self.pressure.lock().unwrap()
    }
    pub fn calc_weight(&self) -> f64 {
        let str: f64 = self.get_pressure() as f64;
        str / M
    }
}

fn pick_event_device() -> anyhow::Result<String> {
    if let Ok(path) = env::var("ASAHI_TRACKPAD_EVENT") {
        return Ok(path);
    }

    let default = "/dev/input/event2";
    if Path::new(default).exists() {
        return Ok(default.to_string());
    }

    if let Ok(entries) = fs::read_dir("/dev/input/by-path") {
        for entry in entries.flatten() {
            let file_name = entry.file_name().to_string_lossy().to_lowercase();
            if file_name.contains("trackpad") && file_name.contains("event") {
                return Ok(entry.path().to_string_lossy().to_string());
            }
        }
    }

    // Fall back to scanning /dev/input/event* and prefer names that look like a touchpad.
    let mut first_event: Option<String> = None;
    if let Ok(entries) = fs::read_dir("/dev/input") {
        for entry in entries.flatten() {
            let file_name = entry.file_name().to_string_lossy().to_string();
            if !file_name.starts_with("event") {
                continue;
            }
            let path = entry.path().to_string_lossy().to_string();
            if first_event.is_none() {
                first_event = Some(path.clone());
            }
            if let Ok(dev) = Device::open(&path) {
                let name = dev.name().unwrap_or("").to_lowercase();
                if name.contains("trackpad") || name.contains("touchpad") {
                    return Ok(path);
                }
            }
        }
    }

    if let Some(path) = first_event {
        return Ok(path);
    }

    bail!(
        "no /dev/input/event* device found; set ASAHI_TRACKPAD_EVENT=/dev/input/eventX (try `ls /dev/input`)"
    )
}
