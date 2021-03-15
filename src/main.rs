/*
Copyright 2021 Aleksandre Sarkisov (Alex Czar)

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
 */

extern crate getopts;
extern crate rusb;

use std::fmt::Display;
use std::fmt::Formatter;
use std::num::ParseIntError;
use std::ops::Range;
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
            Ok(params) => set_params(params),
            Err(f) => eprintln!("Invalid options: {}", f),
        }
    }
}

fn set_params(params: Vec<DeviceParam>) {
    let kraken = Kraken::new();

    kraken.detach();
    for param in params {
        kraken.write(param);
    }
    kraken.reattach();
}

fn validate(opt_matches: Matches) -> Result<Vec<DeviceParam>, ParseIntError> {
    let range: Range<u8> = 10..100;

    let mut params: Vec<DeviceParam> = Vec::new();

    if let Some(fan_speed) = opt_matches.opt_get("f")? {
        assert!(
            range.contains(&fan_speed),
            "FANSPEED should be within range 10-100"
        );
        params.push(FanSpeed(fan_speed))
    }

    if let Some(pump_speed) = opt_matches.opt_get("p")? {
        assert!(
            range.contains(&pump_speed),
            "PUMPSPEED should be within range 10-100"
        );
        params.push(PumpSpeed(pump_speed))
    }

    Ok(params)
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

struct Kraken {
    device: rusb::Device<GlobalContext>,
}

impl Kraken {
    fn new() -> Kraken {
        Kraken {
            device: find_kraken_device(),
        }
    }
}

fn find_kraken_device() -> Device<GlobalContext> {
    let vid = 0x1e71;
    let pid = 0x170e;
    let devices = rusb::devices().expect("Could not enumerate USB devices");
    devices
        .iter()
        .find(|d| {
            let dd = d.device_descriptor().unwrap();
            dd.vendor_id() == vid && dd.product_id() == pid
        })
        .unwrap_or_else(|| {
            panic!(
                "Could not find usb device with VendorId: {} and ProductId: {}.",
                vid, pid
            )
        })
}

trait KrakenControl {
    fn detach(&self);
    fn reattach(self);
    fn write(&self, param: DeviceParam);
}

impl KrakenControl for Kraken {
    fn detach(&self) {
        let mut handle = self.device.open().expect("Could not open device.");
        if handle.kernel_driver_active(0).unwrap() {
            println!("Detaching kernel driver...");
            handle.detach_kernel_driver(0).unwrap()
        }
    }

    fn reattach(self) {
        let mut handle = self.device.open().expect("Could not open device.");
        if !handle.kernel_driver_active(0).unwrap() {
            println!("Re-attaching kernel driver...");
            handle.attach_kernel_driver(0).unwrap();
        }
    }

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

enum DeviceParam {
    FanSpeed(u8),
    PumpSpeed(u8),
}

impl Display for DeviceParam {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FanSpeed(speed) => write!(f, "fan speed to {}%", speed),
            PumpSpeed(speed) => write!(f, "pump speed to {}%", speed),
        }
    }
}

fn write_to_device(device: &rusb::Device<GlobalContext>, buf: &[u8]) {
    let write_result = device
        .open()
        .and_then(|handle| handle.write_bulk(1, buf, Duration::from_secs(1)));
    match write_result {
        Ok(_) => println!("done"),
        Err(f) => eprintln!("Error: {}", f),
    }
}

#[cfg(test)]
#[allow(unused_must_use)]
mod tests;
