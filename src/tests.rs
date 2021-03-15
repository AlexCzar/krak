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

use test_case::test_case;

use super::*;

#[test_case("--fan-speed", 9; "FANSPEED less than range min")]
#[test_case("--fan-speed", 101; "FANSPEED more than range max")]
#[test_case("--pump-speed", 9; "PUMPSPEED less than range min")]
#[test_case("--pump-speed", 101; "PUMPSPEED more than range max")]
fn should_fail_validation_if_speed_is_outside_of_range(test_param: &str, test_value: u8) {
    let opts = build_options();
    let fanspeed = format!("{}={}", test_param, test_value);
    let command_line = vec!["krak", fanspeed.as_str()];
    let result = opts.parse(command_line).map(|r| validate(r)).unwrap();
    assert!(result.is_err());
    if let Err(CliValidationError {
        value,
        param,
        reason,
    }) = result
    {
        assert_eq!(test_value.to_string(), value);
        assert_eq!("out of range", reason);
        assert_eq!(param, param)
    }
}

#[test_case("--fan-speed", 10, FanSpeed(10); "FANSPEED is range.min")]
#[test_case("--fan-speed", 42, FanSpeed(42); "FANSPEED is inside range")]
#[test_case("--fan-speed", 100, FanSpeed(100); "FANSPEED is range.max")]
#[test_case("--pump-speed", 10, PumpSpeed(10); "PUMPSPEED is range.min")]
#[test_case("--pump-speed", 42, PumpSpeed(42); "PUMPSPEED is inside range")]
#[test_case("--pump-speed", 100, PumpSpeed(100); "PUMPSPEED is range.max")]
fn should_pass_validation_if_speed_is_in_range(
    test_param: &str,
    test_value: u8,
    test_opt: DeviceParam,
) {
    let opts = build_options();
    let fanspeed = format!("{}={}", test_param, test_value);
    let command_line = vec!["krak", fanspeed.as_str()];
    let result = opts.parse(command_line).map(|r| validate(r)).unwrap();

    assert!(result.is_ok());
    if let Ok(vec) = result {
        assert_eq!(vec.len(), 1);
        assert_eq!(vec.first().unwrap(), &test_opt);
    }
}
