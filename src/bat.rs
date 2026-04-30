use std::{
    env,
    fs,
    io,
    path::{Path, PathBuf},
};

const DEFAULT_POWER_SUPPLY_DIR: &str = "/sys/class/power_supply/macsmc_battery";
const MICRO: f64 = 1_000_000.0;

pub struct Battery {
    dir: PathBuf,
    pub rm_wh: f64,
    pub watt: f64,
    pub capacity: f64,
    pub charge_ah: f64,
    pub health: f64,
    pub status: String,
}
impl Battery {
    pub fn new() -> anyhow::Result<Self> {
        let dir = pick_power_supply_dir().unwrap_or_else(|| PathBuf::from(DEFAULT_POWER_SUPPLY_DIR));
        let mut battery = Self::unavailable(dir);
        let _ = battery.refresh();
        Ok(battery)
    }

    pub fn refresh(&mut self) -> anyhow::Result<()> {
        let next = Self::from_sysfs(&self.dir)?;
        self.rm_wh = next.rm_wh;
        self.watt = next.watt;
        self.capacity = next.capacity;
        self.charge_ah = next.charge_ah;
        self.health = next.health;
        self.status = next.status;
        Ok(())
    }

    pub fn from_sysfs(dir: impl AsRef<Path>) -> anyhow::Result<Self> {
        let dir = dir.as_ref();

        let capacity = read_f64_opt(dir.join("capacity"))?.unwrap_or(0.0);
        let status = read_string(dir.join("status")).unwrap_or_else(|| "unknown".to_string());

        let rm_wh = if dir.join("energy_now").exists() {
            read_f64(dir.join("energy_now"))? / MICRO
        } else if dir.join("charge_now").exists() && dir.join("voltage_now").exists() {
            // charge_now is typically µAh, voltage_now is µV -> Wh = Ah * V
            let charge_ah = read_f64(dir.join("charge_now"))? / MICRO;
            let voltage_v = read_f64(dir.join("voltage_now"))? / MICRO;
            charge_ah * voltage_v
        } else {
            0.0
        };

        let watt = if dir.join("power_now").exists() {
            read_f64(dir.join("power_now"))? / MICRO
        } else if dir.join("current_now").exists() && dir.join("voltage_now").exists() {
            // current_now is typically µA, voltage_now is µV -> W = A * V
            let current_a = read_f64(dir.join("current_now"))? / MICRO;
            let voltage_v = read_f64(dir.join("voltage_now"))? / MICRO;
            current_a * voltage_v
        } else {
            0.0
        };

        let charge_ah = if dir.join("charge_now").exists() {
            read_f64(dir.join("charge_now"))? / MICRO
        } else {
            0.0
        };

        let health = if dir.join("energy_full").exists() && dir.join("energy_full_design").exists() {
            let full = read_f64(dir.join("energy_full"))?;
            let design = read_f64(dir.join("energy_full_design"))?;
            if design > 0.0 {
                (full / design) * 100.0
            } else {
                0.0
            }
        } else if dir.join("charge_full").exists() && dir.join("charge_full_design").exists() {
            let full = read_f64(dir.join("charge_full"))?;
            let design = read_f64(dir.join("charge_full_design"))?;
            if design > 0.0 {
                (full / design) * 100.0
            } else {
                0.0
            }
        } else {
            0.0
        };

        Ok(Self {
            dir: dir.to_path_buf(),
            rm_wh,
            watt,
            capacity,
            charge_ah,
            health,
            status,
        })
    }

    fn unavailable(dir: PathBuf) -> Self {
        Self {
            dir,
            rm_wh: 0.0,
            watt: 0.0,
            capacity: 0.0,
            charge_ah: 0.0,
            health: 0.0,
            status: "unavailable".to_string(),
        }
    }
}

fn read_f64(path: impl AsRef<Path>) -> anyhow::Result<f64> {
    let s = fs::read_to_string(&path)?;
    Ok(s.trim().parse::<f64>()?)
}

fn read_f64_opt(path: impl AsRef<Path>) -> anyhow::Result<Option<f64>> {
    match fs::read_to_string(&path) {
        Ok(s) => Ok(Some(s.trim().parse::<f64>()?)),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e.into()),
    }
}

fn read_string(path: impl AsRef<Path>) -> Option<String> {
    fs::read_to_string(path).ok().map(|s| s.trim().to_string())
}

fn pick_power_supply_dir() -> Option<PathBuf> {
    if let Ok(dir) = env::var("ASAHI_BATTERY_DIR") {
        return Some(PathBuf::from(dir));
    }

    let default = PathBuf::from(DEFAULT_POWER_SUPPLY_DIR);
    if default.exists() {
        return Some(default);
    }

    let root = Path::new("/sys/class/power_supply");
    let entries = fs::read_dir(root).ok()?;
    for entry in entries.flatten() {
        let path = entry.path();
        if read_string(path.join("type")).as_deref() == Some("Battery") {
            return Some(path);
        }
    }
    None
}
