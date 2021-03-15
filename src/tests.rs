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

#[cfg(test)]
use super::*;

#[test]
#[should_panic(expected = "FANSPEED should be within range 10-100")]
fn fan_speed_should_fail_validation_if_value_less_than_range() {
    let opts = build_options();
    let command_line = vec!["krak", "-f9"];
    opts.parse(command_line).map(|r| validate(r));
}

#[test]
#[should_panic(expected = "FANSPEED should be within range 10-100")]
fn fan_speed_should_fail_validation_if_value_more_than_range() {
    let opts = build_options();
    let command_line = vec!["krak", "-f101"];
    opts.parse(command_line).map(|r| validate(r));
}

#[test]
#[should_panic(expected = "PUMPSPEED should be within range 10-100")]
fn pump_speed_should_fail_validation_if_value_less_than_range() {
    let opts = build_options();
    let command_line = vec!["krak", "-p9"];
    opts.parse(command_line).map(|r| validate(r));
}

#[test]
#[should_panic(expected = "PUMPSPEED should be within range 10-100")]
fn pump_speed_should_fail_validation_if_value_more_than_range() {
    let opts = build_options();
    let command_line = vec!["krak", "-p101"];
    opts.parse(command_line).map(|r| validate(r));
}
