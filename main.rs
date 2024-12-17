
use std::{thread,time::Duration};
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() {
    struct Clock{
        hour: i32,
        minute: i32,
        sec: i32,
    }

    let mut clock = Clock{
        hour: 2,
        minute: 0,
        sec: 0,
    };
    let hour = clock.hour * 3600;
    let minute = clock.minute * 60;
    let sec = clock.sec;
    let mut sum:i32 = hour + minute + sec;
    
    let file = File::open("pic.pic").unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();

    reader.read_to_string(&mut buffer).unwrap();

    while sum >= 0 {
        //Clear the screen
        println!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("{}", buffer);
        println!("{:02}:{:02}:{:02}", clock.hour, clock.minute,clock.sec);
        clock.sec = clock.sec - 1;
        if clock.sec < 0 {
            clock.sec = 59;
            clock.minute = clock.minute - 1;
        }
        if clock.minute < 0{
            clock.minute = 59;
            clock.hour = clock.hour - 1;
        }
        sum = sum - 1;
        thread::sleep(Duration::from_secs(1));
    }
}
