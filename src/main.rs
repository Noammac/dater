extern crate chrono;
extern crate termion;

use chrono::prelude::{Date, Datelike, Local, NaiveDate, TimeZone};
use std::env;
use termion::{color, style};

fn main() {
    let now: Date<Local> = Local::now().date();

    let ord = now.ordinal();
    let year = now.year();

    let underline = style::Underline;
    let bold = style::Bold;
    let reset = style::Reset;

    let msg: String = match env::var("COUNTDOWN_DATE") {
        Ok(val) => match NaiveDate::parse_from_str(&val, "%F") {
            Ok(dt) => {
                format!(
                    " {red}{bold}{delta}{reset} days left",
                    delta = (Local.from_utc_date(&dt) - now).num_days(),
                    red = color::Fg(color::Red),
                    bold = bold,
                    reset = reset
                )
            }
            Err(_) => {
                format!(
                    " {bold}ERROR:{reset} misformatted countdown date {date}",
                    bold = bold,
                    reset = reset,
                    date = val
                )
            }
        },
        Err(_) => String::new(),
    };

    println!(
        "Today is the {underline}{day}{reset} day of the year {bold}{year}{reset}, or {underline}{ord:#X}{reset} in hex.{msg}",
        underline = underline,
        bold = bold,
        reset = reset,
        day = ordinate(ord),
        year = year,
        ord = ord,
        msg = msg
    );

    if ord == 256 {
        println!(
            "Happy {bold}Day of the Programmer{reset}!",
            bold = bold,
            reset = reset
        );
    }
    if (now.month() == 11) & (now.day() == 20) {
        println!(
            "Happy birthday! You are now {invert}{bold}{purple}{age}{reset} years old!",
            invert = style::Invert,
            bold = bold,
            purple = color::Fg(color::Magenta),
            reset = reset,
            age = year - 2002
        );
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
