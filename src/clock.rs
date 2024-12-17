
use std::{thread,time::Duration};

pub fn clock(number: i32) -> String {
    struct Clock{
        hour: i32,
        minute: i32,
        sec: i32,
    }
    let second1 = number % 60;
    let minute1 = (number / 60) % 60;
    let hour1= (number / 60) / 60;

    let mut clock = Clock{
        hour: hour1,
        minute: minute1,
        sec: second1,
    };

    let parsed = format!("{:02}:{:02}:{:02}", clock.hour, clock.minute,clock.sec);
    return parsed.to_string();
}
