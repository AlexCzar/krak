extern crate getopts;
extern crate rusb;

use std::fmt::Display;
use std::fmt::Formatter;
use std::num::ParseIntError;
use std::time::Duration;

use getopts::{Matches, Options};
use rusb::{Device, GlobalContext};

use crate::DeviceParam::{FanSpeed, PumpSpeed};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();
    let opts = build_options();
    let opt_matches = match_opts(args, &opts);

    if opt_matches.opt_present("h") || no_options_present(&opt_matches) {
        print_usage(program, opts)
    } else {
        match validate(opt_matches) {
            Ok((fan_speed, pump_speed)) => set_speeds(fan_speed, pump_speed),
            Err(f) => eprintln!("Invalid options: {}", f),
        };
    }
}

fn set_speeds(fan_speed: Option<u8>, pump_speed: Option<u8>) {
    let kraken = Krak {
        device: find_kraken(),
    };

    match kraken.detach() {
        Ok(_) => {
            kraken.set_fan_speed(fan_speed);
            kraken.set_pump_speed(pump_speed);
            kraken.reattach().unwrap();
        }
        Err(f) => eprintln!("AAARGH!!! {}", f),
    }
}

fn validate(opt_matches: Matches) -> Result<(Option<u8>, Option<u8>), ParseIntError> {
    let fan_speed = opt_matches.opt_get("f")?;
    if let Some(f) = fan_speed {
        assert!(f <= 100 && f >= 10, "FANSPEED should be with range 10-100");
    }

    let pump_speed = opt_matches.opt_get("p")?;
    if let Some(p) = pump_speed {
        assert!(p <= 100 && p >= 10, "PUMPSPEED should be with range 10-100");
    }
    Result::Ok((fan_speed, pump_speed))
}

fn no_options_present(opt_matches: &Matches) -> bool {
    !opt_matches.opt_present("f") && !opt_matches.opt_present("p")
}

fn print_usage(program: String, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn match_opts(args: Vec<String>, opts: &Options) -> Matches {
    match opts.parse(&args[1..]) {
        Ok(matches) => matches,
        Err(failure) => {
            panic!(failure.to_string())
        }
    }
}

fn build_options() -> Options {
    let mut opts = Options::new();
    opts.optflag("h", "help", "Print usage.");
    opts.optopt("f", "fan-speed", "Fan speed in 10 - 100 range.", "FANSPEED");
    opts.optopt(
        "p",
        "pump-speed",
        "Pump speed in 10 - 100 range.",
        "PUMPSEED",
    );
    opts
}

struct Krak {
    device: rusb::Device<GlobalContext>,
}

impl Krak {}

trait KrakControl {
    fn detach(&self) -> rusb::Result<()>;
    fn reattach(self) -> rusb::Result<()>;
    fn set_fan_speed(&self, fan_speed: Option<u8>);
    fn set_pump_speed(&self, pump_speed: Option<u8>);
}

impl KrakControl for Krak {
    fn detach(&self) -> rusb::Result<()> {
        self.device.open().and_then(|mut handle| {
            if handle.kernel_driver_active(0).unwrap() {
                println!("Detaching kernel driver...");
                handle.detach_kernel_driver(0)
            } else {
                Ok(())
            }
        })
    }

    fn reattach(self) -> rusb::Result<()> {
        self.device.open().and_then(|mut handle| {
            if !handle.kernel_driver_active(0).unwrap() {
                println!("Re-attaching kernel driver...");
                handle.attach_kernel_driver(0).unwrap();
            };
            Ok(())
        })
    }

    fn set_fan_speed(&self, fan_speed: Option<u8>) {
        if let Some(fan_speed) = fan_speed {
            self.write(FanSpeed(fan_speed));
        }
    }

    fn set_pump_speed(&self, pump_speed: Option<u8>) {
        if let Some(pump_speed) = pump_speed {
            self.write(PumpSpeed(pump_speed))
        }
    }
}

fn find_kraken() -> Device<GlobalContext> {
    let vid = 0x1e71;
    let pid = 0x170e;
    let devices = rusb::devices().expect("Could not enumerate USB devices");
    devices
        .iter()
        .find(|d| {
            let dd = d.device_descriptor().unwrap();
            let condition = dd.vendor_id() == vid && dd.product_id() == pid;
            condition
        })
        .expect(
            format!(
                "Could not find usb device with VendorId: {} and ProductId: {}.",
                vid, pid
            )
            .as_str(),
        )
}

enum DeviceParam {
    FanSpeed(u8),
    PumpSpeed(u8),
}

impl Display for DeviceParam {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FanSpeed(speed) => {
                write!(f, "fan speed to {}%", speed)
            }
            PumpSpeed(speed) => {
                write!(f, "pump speed to {}%", speed)
            }
        }
    }
}

impl Krak {
    fn write(&self, param: DeviceParam) {
        println!("Setting {}.", param);
        let buf = match param {
            DeviceParam::FanSpeed(speed) => vec![
                2, 77, 0, 0, speed, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            DeviceParam::PumpSpeed(speed) => vec![
                2, 77, 64, 0, speed, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
        };
        write_to_device(&self.device, &buf)
    }
}

fn write_to_device(device: &rusb::Device<GlobalContext>, buf: &[u8]) {
    device
        .open()
        .and_then(|handle| {
            handle.write_bulk(1, buf, Duration::from_secs(1)).unwrap();
            Ok(())
        })
        .unwrap();
}

#[cfg(test)]
#[allow(unused_must_use)]
mod tests;
