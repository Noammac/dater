extern crate chrono;
extern crate termion;

use chrono::prelude::{Date, Datelike, Local, NaiveDate, TimeZone};
use std::env;
use termion::{color, style};

fn main() {
    let now: Date<Local> = Local::now().date();

    let ord = now.ordinal();
    let year = now.year();

    let msg: String = match env::var("COUNTDOWN_DATE") {
        Ok(val) => match NaiveDate::parse_from_str(&val, "%F") {
            Ok(dt) => {
                format!(
                    " {red}{bold}{delta}{reset} days left",
                    delta = (Local.from_utc_date(&dt) - now).num_days(),
                    red = color::Fg(color::Red),
                    bold = style::Bold,
                    reset = style::Reset
                )
            }
            Err(_) => {
                format!(" ERROR: misformatted countdown date {}", val)
            }
        },
        Err(_) => String::new(),
    };

    println!(
        "Today is the {} day of the year {}, or {:#X} in hex.{}",
        ordinate(ord),
        year,
        ord,
        msg
    );

    if ord == 256 {
        println!("Happy day of the Programmer!");
    }
    if (now.month() == 11) & (now.day() == 20) {
        println!("Happy birthday! You are now {} years old!", year - 2002);
    }
}

fn ordinate(num: u32) -> String {
    let suff = match num % 100 {
        _a @ 4..=20 => ("th"),
        dec => match dec % 10 {
            1 => ("st"),
            2 => ("nd"),
            3 => ("rd"),
            _ => "th",
        },
    };
    format!("{}{}", num, suff)
}
