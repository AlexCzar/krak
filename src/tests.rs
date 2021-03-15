#[cfg(test)]
use super::*;

#[test]
#[should_panic(expected = "FANSPEED should be with range 10-100")]
fn fan_speed_should_fail_validation_if_value_less_than_range() {
    let opts = build_options();
    let command_line = vec!["krak", "-f9"];
    opts.parse(command_line).map(|r| validate(r));
}

#[test]
#[should_panic(expected = "FANSPEED should be with range 10-100")]
fn fan_speed_should_fail_validation_if_value_more_than_range() {
    let opts = build_options();
    let command_line = vec!["krak", "-f101"];
    opts.parse(command_line).map(|r| validate(r));
}

#[test]
#[should_panic(expected = "PUMPSPEED should be with range 10-100")]
fn pump_speed_should_fail_validation_if_value_less_than_range() {
    let opts = build_options();
    let command_line = vec!["krak", "-p9"];
    opts.parse(command_line).map(|r| validate(r));
}

#[test]
#[should_panic(expected = "PUMPSPEED should be with range 10-100")]
fn pump_speed_should_fail_validation_if_value_more_than_range() {
    let opts = build_options();
    let command_line = vec!["krak", "-p101"];
    opts.parse(command_line).map(|r| validate(r));
}
